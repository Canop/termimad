use crate::area::{terminal_size, Area};
use crate::color::*;
use crate::composite::FmtComposite;
use crate::compound_style::CompoundStyle;
use crate::inline::FmtInline;
use crate::line::FmtLine;
use crate::line_style::LineStyle;
use crate::scrollbar_style::ScrollBarStyle;
use crate::spacing::Spacing;
use crate::styled_char::StyledChar;
use crate::tbl::*;
use crate::text::FmtText;

use crossterm::{self, Attribute, Color};
use minimad::{Alignment, Composite, CompositeStyle, Compound, Line, MAX_HEADER_DEPTH};
use std::{self, fmt};

/// A skin defining how a parsed mardkown appears on the terminal
/// (fg and bg colors, bold, italic, underline, etc.)
pub struct MadSkin {
    pub paragraph: LineStyle,
    pub bold: CompoundStyle,
    pub italic: CompoundStyle,
    pub strikeout: CompoundStyle,
    pub inline_code: CompoundStyle,
    pub code_block: LineStyle,
    pub headers: [LineStyle; MAX_HEADER_DEPTH],
    pub scrollbar: ScrollBarStyle,
    pub table: LineStyle, // the compound style is for border chars
    pub bullet: StyledChar,
    pub quote_mark: StyledChar,
    pub horizontal_rule: StyledChar,
}

impl Default for MadSkin {
    /// Build a customizable skin.
    ///
    /// It's initialized with sensible monochrome settings.
    fn default() -> MadSkin {
        let mut skin = MadSkin {
            paragraph: LineStyle::default(),
            bold: CompoundStyle::new(Some(Color::White), None, vec![Attribute::Bold]),
            italic: CompoundStyle::with_attr(Attribute::Italic),
            strikeout: CompoundStyle::with_attr(Attribute::CrossedOut),
            inline_code: CompoundStyle::with_bg(gray(4)),
            code_block: LineStyle::default(),
            headers: Default::default(),
            scrollbar: ScrollBarStyle::new(),
            table: LineStyle {
                compound_style: CompoundStyle::with_fg(gray(7)),
                align: Alignment::Unspecified,
            },
            bullet: StyledChar::from_fg_char(gray(8), '•'),
            quote_mark: StyledChar::new(
                CompoundStyle::new(Some(gray(12)), None, vec![Attribute::Bold]),
                '▐',
            ),
            horizontal_rule: StyledChar::from_fg_char(gray(6), '―'),
        };
        skin.code_block.set_bg(gray(4));
        for h in &mut skin.headers {
            h.add_attr(Attribute::Underlined);
        }
        skin.headers[0].add_attr(Attribute::Bold);
        skin.headers[0].align = Alignment::Center;
        skin.headers[0].set_fg(gray(22));
        skin.headers[1].set_fg(gray(21));
        skin.headers[2].set_fg(gray(20));
        skin
    }
}

impl MadSkin {
    /// Set a common foregreound color for all header levels
    ///
    /// (it's still possible to change them individually with
    /// skin.headers[i])
    pub fn set_headers_fg(&mut self, c: Color) {
        for h in &mut self.headers {
            h.set_fg(c);
        }
    }

    /// Set a common background color for all header levels
    ///
    /// (it's still possible to change them individually with
    /// skin.headers[i])
    pub fn set_headers_bg(&mut self, c: Color) {
        for h in &mut self.headers {
            h.set_bg(c);
        }
    }

    /// Return the number of visible chars in a composite
    pub fn visible_composite_length(&self, composite: &Composite<'_>) -> usize {
        (match composite.style {
            CompositeStyle::ListItem => 2, // space of the bullet
            CompositeStyle::Quote => 2,    // space of the quoting char
            _ => 0,
        }) + composite.char_length()
    }

    pub fn visible_line_length(&self, line: &Line<'_>) -> usize {
        match line {
            Line::Normal(composite) => self.visible_composite_length(composite),
            _ => 0, // FIXME implement
        }
    }

    /// return the style to apply to a given line
    fn line_style(&self, style: &CompositeStyle) -> &LineStyle {
        match style {
            CompositeStyle::Code => &self.code_block,
            CompositeStyle::Header(level) if *level <= MAX_HEADER_DEPTH as u8 => {
                &self.headers[*level as usize - 1]
            }
            _ => &self.paragraph,
        }
    }

    /// return the style appliable to a given compound.
    /// It's a composition of the various appliable base styles.
    fn compound_style(&self, line_style: &LineStyle, compound: &Compound<'_>) -> CompoundStyle {
        let mut os = line_style.compound_style.clone();
        if compound.italic {
            os.overwrite_with(&self.italic);
        }
        if compound.strikeout {
            os.overwrite_with(&self.strikeout);
        }
        if compound.bold {
            os.overwrite_with(&self.bold);
        }
        if compound.code {
            os.overwrite_with(&self.inline_code);
        }
        os
    }

    // return a formatted line or part of line.
    //
    // Don't use this function if `src` is expected to be several lines.
    pub fn inline<'k, 's>(&'k self, src: &'s str) -> FmtInline<'k, 's> {
        let composite = FmtComposite::from(Composite::from_inline(src), self);
        FmtInline {
            skin: self,
            composite,
        }
    }

    /// return a formatted text.
    ///
    /// Code blocs will be right justified
    pub fn text<'k, 's>(&'k self, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        FmtText::from(self, src, width)
    }

    /// return a formatted text, with lines wrapped or justified for the current terminal
    /// width.
    ///
    /// Code blocs will be right justified
    pub fn term_text<'k, 's>(&'k self, src: &'s str) -> FmtText<'k, 's> {
        let (width, _) = terminal_size();
        FmtText::from(self, src, Some(width as usize))
    }

    /// return a formatted text, with lines wrapped or justified for the
    /// passed area width (with space for a scrollbar).
    ///
    /// Code blocs will be right justified
    pub fn area_text<'k, 's>(&'k self, src: &'s str, area: &Area) -> FmtText<'k, 's> {
        FmtText::from(self, src, Some(area.width as usize))
    }

    pub fn print_inline(&self, src: &str) {
        print!("{}", self.inline(src));
    }
    pub fn print_text(&self, src: &str) {
        print!("{}", self.term_text(src));
    }

    /// Write a composite.
    ///
    /// This function is internally used and normally not needed outside
    ///  of Termimad's implementation.
    pub fn write_fmt_composite(
        &self,
        f: &mut fmt::Formatter<'_>,
        fc: &FmtComposite<'_>,
        outer_width: Option<usize>,
        with_right_completion: bool,
    ) -> fmt::Result {
        let ls = self.line_style(&fc.composite.style);
        let (lpi, rpi) = fc.completions(); // inner completion
        let inner_width = fc.spacing.map_or(fc.visible_length, |sp| sp.width);
        let (lpo, rpo) = Spacing::optional_completions(ls.align, inner_width, outer_width);
        self.paragraph.repeat_space(f, lpo)?;
        ls.compound_style.repeat_space(f, lpi)?;
        if fc.composite.is_list_item() {
            write!(f, "{} ", self.bullet)?;
        }
        if fc.composite.is_quote() {
            write!(f, "{} ", self.quote_mark)?;
        }
        for c in &fc.composite.compounds {
            let os = self.compound_style(ls, c);
            write!(f, "{}", os.apply_to(c.as_str()))?;
        }
        ls.compound_style.repeat_space(f, rpi)?;
        if with_right_completion {
            self.paragraph.repeat_space(f, rpo)?;
        }
        Ok(())
    }

    /// Write a line in the passed formatter, with completions.
    ///
    /// Right completion is optional because:
    /// - if a text isn't right completed it shrinks better when you reduce the width
    ///   of the terminal
    /// - right completion is useful to overwrite previous rendering without
    ///   flickering (in scrollable views)
    pub fn write_fmt_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        line: &FmtLine<'_>,
        width: Option<usize>,
        with_right_completion: bool,
    ) -> fmt::Result {
        match line {
            FmtLine::Normal(fc) => {
                self.write_fmt_composite(f, fc, width, with_right_completion)?;
            }
            FmtLine::TableRow(FmtTableRow { cells }) => {
                let tbl_width = 1 + cells.iter().fold(0, |sum, cell| {
                    if let Some(spacing) = cell.spacing {
                        sum + spacing.width + 1
                    } else {
                        sum + cell.visible_length + 1
                    }
                });
                let (lpo, rpo) = Spacing::optional_completions(self.table.align, tbl_width, width);
                self.paragraph.repeat_space(f, lpo)?;
                for cell in cells {
                    write!(f, "{}", self.table.compound_style.apply_to("│"))?;
                    self.write_fmt_composite(f, &cell, None, false)?;
                }
                write!(f, "{}", self.table.compound_style.apply_to("│"))?;
                if with_right_completion {
                    self.paragraph.repeat_space(f, rpo)?;
                }
            }
            FmtLine::TableRule(rule) => {
                let tbl_width = 1 + rule.widths.iter().fold(0, |sum, w| sum + w + 1);
                let (lpo, rpo) = Spacing::optional_completions(self.table.align, tbl_width, width);
                self.paragraph.repeat_space(f, lpo)?;
                write!(
                    f,
                    "{}",
                    self.table.compound_style.apply_to(match rule.position {
                        RelativePosition::Top => '┌',
                        RelativePosition::Other => '├',
                        RelativePosition::Bottom => '└',
                    })
                )?;
                for (idx, &width) in rule.widths.iter().enumerate() {
                    if idx > 0 {
                        write!(
                            f,
                            "{}",
                            self.table.compound_style.apply_to(match rule.position {
                                RelativePosition::Top => '┬',
                                RelativePosition::Other => '┼',
                                RelativePosition::Bottom => '┴',
                            })
                        )?;
                    }
                    self.table.repeat_string(f, "─", width)?;
                }
                write!(
                    f,
                    "{}",
                    self.table.compound_style.apply_to(match rule.position {
                        RelativePosition::Top => '┐',
                        RelativePosition::Other => '┤',
                        RelativePosition::Bottom => '┘',
                    })
                )?;
                if with_right_completion {
                    self.paragraph.repeat_space(f, rpo)?;
                }
            }
            FmtLine::HorizontalRule => {
                if let Some(w) = width {
                    write!(f, "{}", self.horizontal_rule.repeated(w))?;
                }
            }
        }
        Ok(())
    }
}
