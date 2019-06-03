use crate::area::Area;
use crate::composite::FmtComposite;
use crate::inline::FmtInline;
use crate::line::FmtLine;
use crate::text::FmtText;
use crate::spacing::Spacing;
use crate::style::*;
use crate::tbl::*;

use crossterm::{Attribute, Color, Terminal};
use minimad::{Alignment, Compound, Composite, CompositeStyle, Line, MAX_HEADER_DEPTH};
use std::{self, fmt};


/// A skin defining how a parsed mardkown appears on the terminal
/// (fg and bg colors, bold, italic, underline, etc.)
pub struct MadSkin {
    pub paragraph: LineStyle,
    pub bold: CompoundStyle,
    pub italic: CompoundStyle,
    pub code: LineStyle,
    pub headers: [LineStyle; MAX_HEADER_DEPTH],
    pub scrollbar: ScrollBarStyle,
    pub table_border: CompoundStyle,
}

impl Default for MadSkin {
    /// build a customizable skin.
    /// It's initialized with sensible monochrome settings.
    fn default() -> MadSkin {
        let mut skin = MadSkin {
            paragraph: LineStyle::default(),
            bold: CompoundStyle::new(Some(Color::White), None, vec![Attribute::Bold]),
            italic: CompoundStyle::with_attr(Attribute::Italic),
            code: LineStyle::default(),
            headers: Default::default(),
            scrollbar: ScrollBarStyle::new(),
            table_border: CompoundStyle::with_fg(rgb!(110, 110, 110)),
        };
        skin.code.set_bg(rgb!(40, 40, 40));
        for h in &mut skin.headers {
            h.add_attr(Attribute::Underlined);
        }
        skin.headers[0].add_attr(Attribute::Bold);
        skin.headers[0].align = Alignment::Center;
        skin.headers[0].set_fg(Color::Rgb{r:250, g:250, b:250});
        skin.headers[1].set_fg(Color::Rgb{r:240, g:240, b:240});
        skin
    }
}

impl MadSkin {
    pub fn set_headers_fg(&mut self, c: Color) {
        for h in &mut self.headers {
            h.set_fg(c);
        }
    }
    pub fn set_headers_bg(&mut self, c: Color) {
        for h in &mut self.headers {
            h.set_bg(c);
        }
    }

    pub fn visible_composite_length(&self, composite: &Composite<'_>) -> usize {
        (match composite.style {
            CompositeStyle::ListItem => 2, // space of the bullet
            _ => 0,
        }) + composite.char_length()
    }

    pub fn visible_line_length(&self, line: &Line<'_>) -> usize {
        match line {
            Line::Normal( composite ) => self.visible_composite_length(composite),
            _ => 0, // FIXME implement
        }
    }

    /// return the style appliable to a given line
    fn line_style(&self, style: &CompositeStyle) -> &LineStyle {
        match style {
            CompositeStyle::Code => &self.code,
            CompositeStyle::Header(level) if *level <= MAX_HEADER_DEPTH as u8 => {
                &self.headers[*level as usize - 1]
            }
            _ => &self.paragraph,
        }
    }

    /// return the style appliable to a given compound.
    /// It's a composition of the various appliable base styles.
    fn compound_style(
        &self,
        line_style: &LineStyle,
        compound: &Compound<'_>,
    ) -> CompoundStyle {
        let mut os = line_style.compound_style.clone();
        if compound.italic {
            os.overwrite_with(&self.italic);
        }
        if compound.bold {
            os.overwrite_with(&self.bold);
        }
        if compound.code {
            os.overwrite_with(&self.code.compound_style);
        }
        os
    }

    // return a formatted line or part of line.
    // Don't use this function if `src` is expected to be several lines.
    pub fn inline<'k, 's>(&'k self, src: &'s str) -> FmtInline<'k, 's> {
        let composite = FmtComposite::from(
            Composite::from_inline(src),
            self
        );
        FmtInline {
            skin: self,
            composite,
        }
    }
    /// return a formatted text.
    /// Code blocs will be right justified
    pub fn text<'k, 's>(&'k self, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        FmtText::from(self, src, width)
    }

    /// return a formatted text, with lines wrapped or justified for the current terminal
    /// width.
    /// Code blocs will be right justified
    pub fn term_text<'k, 's>(&'k self, src: &'s str) -> FmtText<'k, 's> {
        let (width, _) = Terminal::new().terminal_size();
        FmtText::from(self, src, Some(width as usize))
    }
    /// return a formatted text, with lines wrapped or justified for the
    /// passed area width (with space for a scrollbar).
    /// Code blocs will be right justified
    pub fn area_text<'k, 's>(&'k self, src: &'s str, area: &Area) -> FmtText<'k, 's> {
        FmtText::from(self, src, Some(area.width as usize-1))
    }

    pub fn print_inline(&self, src: &str) {
        print!("{}", self.inline(src));
    }
    pub fn print_text(&self, src: &str) {
        println!("{}", self.term_text(src));
    }

    pub fn write_fmt_composite(
        &self,
        f: &mut fmt::Formatter<'_>,
        fc: &FmtComposite<'_>,
        outer_width: Option<usize>,
    ) -> fmt::Result {
        let ls = self.line_style(&fc.composite.style);
        let (lpi, rpi) = fc.completions(); // inner completion
        let inner_width = fc.spacing
            .map_or(fc.visible_length, |sp| sp.width);
        let (lpo, rpo) = match outer_width {
            Some(outer_width) => Spacing::completions(ls.align, inner_width, outer_width),
            None => (0, 0),
        };
        let ospace = self.paragraph.compound_style.apply_to(" ");
        let ispace = ls.compound_style.apply_to(" ");
        for _ in 0..lpo {
            write!(f, "{}", &ospace)?;
        }
        for _ in 0..lpi {
            write!(f, "{}", &ispace)?;
        }
        if fc.composite.is_list_item() {
            write!(f, "• ")?;
        }
        for c in &fc.composite.compounds {
            let os = self.compound_style(ls, c);
            write!(f, "{}", os.apply_to(c.as_str()))?;
        }
        for _ in 0..rpi {
            write!(f, "{}", &ispace)?;
        }
        for _ in 0..rpo {
            write!(f, "{}", &ospace)?;
        }
        Ok(())
    }

    pub fn write_fmt_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        line: &FmtLine<'_>,
        width: Option<usize>,
    ) -> fmt::Result {
        match line {
            FmtLine::Normal(fc) => {
                self.write_fmt_composite(f, fc, width)?;
                writeln!(f)?;
            }
            FmtLine::TableRow(FmtTableRow{cells}) => {
                for cell in cells {
                    write!(f, "{}", self.table_border.apply_to("│"))?;
                    self.write_fmt_composite(f, &cell, None)?;
                }
                writeln!(f, "{}", self.table_border.apply_to("│"))?;
            }
            FmtLine::TableRule(rule) => {
                write!(f, "{}", self.table_border.apply_to(match rule.position {
                    RelativePosition::Top => '┌',
                    RelativePosition::Other => '├',
                    RelativePosition::Bottom => '└',
                }))?;
                for (idx, width) in rule.widths.iter().enumerate() {
                    if idx > 0 {
                        write!(f, "{}", self.table_border.apply_to(match rule.position {
                            RelativePosition::Top => '┬',
                            RelativePosition::Other => '┼',
                            RelativePosition::Bottom => '┴',
                        }))?;
                    }
                    let c = self.table_border.apply_to("─");
                    for _ in 0..*width {
                        write!(f, "{}", c)?;
                    }
                }
                writeln!(f, "{}", self.table_border.apply_to(match rule.position {
                    RelativePosition::Top => '┐',
                    RelativePosition::Other => '┤',
                    RelativePosition::Bottom => '┘',
                }))?;
            }
        }
        Ok(())
    }
}

