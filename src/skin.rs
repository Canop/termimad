use crate::area::Area;
use crate::composite::FmtComposite;
use crate::inline::FmtInline;
use crate::line::FmtLine;
use crate::text::FmtText;
use crate::wrap;
use crate::tbl::*;

use crossterm::{Attribute, Color, ObjectStyle, StyledObject, Terminal};
use minimad::{Compound, Composite, CompositeStyle, Line, MAX_HEADER_DEPTH, TableRow, Text};
use std::{self, fmt};

// The scrollbar style is defined by two styled chars, one
//  for the track, and one for the thumb.
// For the default styling only the fg color is defined
//  and the char is ▐ but everything can be changed
pub struct ScrollBarStyle {
    pub track: StyledObject<char>,
    pub thumb: StyledObject<char>,
}

impl ScrollBarStyle {
    pub fn new() -> ScrollBarStyle {
        let char = '▐';
        ScrollBarStyle {
            track: ObjectStyle::new().fg(Color::Rgb{r:35, g:35, b:35}).apply_to(char),
            thumb: ObjectStyle::new().fg(Color::Rgb{r:140, g:140, b:140}).apply_to(char),
        }
    }
    pub fn set_thumb_fg(&mut self, c: Color) {
        let os = ObjectStyle::new().fg(c);
        self.thumb = os.apply_to(self.thumb.content);
    }
    pub fn set_track_fg(&mut self, c: Color) {
        let os = ObjectStyle::new().fg(c);
        self.track = os.apply_to(self.track.content);
    }
}

/// A skin defining how a parsed mardkown appears on the terminal
/// (fg and bg colors, bold, italic, underline, etc.)
pub struct MadSkin {
    pub paragraph: ObjectStyle,
    pub bold: ObjectStyle,
    pub italic: ObjectStyle,
    pub code: ObjectStyle,
    pub headers: [ObjectStyle; MAX_HEADER_DEPTH],
    pub scrollbar: ScrollBarStyle,
}

// overwrite style of a with b
fn add_style(a: &mut ObjectStyle, b: &ObjectStyle) {
    a.fg_color = b.fg_color.or(a.fg_color);
    a.bg_color = b.bg_color.or(a.bg_color);
    a.attrs.extend(&b.attrs);
}

#[macro_export]
macro_rules! mad_fg {
    (
        $item:expr, $color:expr
    ) => {
        $item.fg_color = Some($color);
    }
}

#[macro_export]
macro_rules! mad_bg {
    (
        $item:expr, $color:expr
    ) => {
        $item.bg_color = Some($color);
    }
}

#[macro_export]
macro_rules! mad_colors {
    (
        $item:expr, $fg:expr, $bg:expr
    ) => {
        $item.fg_color = Some($fg);
        $item.bg_color = Some($bg);
    }
}

impl MadSkin {
    /// build a customizable skin.
    /// It's initialized with sensible monochrome settings.
    pub fn new() -> MadSkin {
        let mut skin = MadSkin {
            paragraph: ObjectStyle::new(),
            bold: ObjectStyle::new().fg(Color::White),
            italic: ObjectStyle::new(),
            code: ObjectStyle::new(),
            headers: Default::default(),
            scrollbar: ScrollBarStyle::new(),
        };
        skin.bold.add_attr(Attribute::Bold);
        skin.italic.add_attr(Attribute::Italic);
        skin.code.bg_color = Some(Color::Rgb {
            r: 40,
            g: 40,
            b: 40,
        });
        for h in &mut skin.headers {
            h.add_attr(Attribute::Underlined);
        }
        skin.headers[0].add_attr(Attribute::Bold);
        skin.headers[0].fg_color = Some(Color::Rgb{r:250, g:250, b:250});
        skin.headers[1].fg_color = Some(Color::Rgb{r:240, g:240, b:240});
        skin
    }
    pub fn set_headers_fg_color(&mut self, c: Color) {
        for h in &mut self.headers {
            h.fg_color = Some(c);
        }
    }
    pub fn set_headers_bg_color(&mut self, c: Color) {
        for h in &mut self.headers {
            h.bg_color = Some(c);
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

    fn composite_object_style(&self, style: &CompositeStyle) -> &ObjectStyle {
        match style {
            CompositeStyle::Code => &self.code,
            CompositeStyle::Header(level) if *level <= MAX_HEADER_DEPTH as u8 => {
                &self.headers[*level as usize - 1]
            }
            _ => &self.paragraph,
        }
    }

    fn compound_object_style(
        &self,
        line_object_style: &ObjectStyle,
        compound: &Compound,
    ) -> ObjectStyle {
        let mut os = line_object_style.clone();
        if compound.italic {
            add_style(&mut os, &self.italic);
        }
        if compound.bold {
            add_style(&mut os, &self.bold);
        }
        if compound.code {
            add_style(&mut os, &self.code);
        }
        os
    }

    // return a formatted line or part of line
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
    //pub fn inline<'s, 'l>(&'s self, src: &'l str) -> FormattedLine<'s, 'l> {
    //    let composite = Composite::from_inline(src);
    //    let line = Line::Normal( composite );
    //    FormattedLine{
    //        skin: self,
    //        line,
    //    }
    //}
    /// return a formatted text.
    /// Code blocs will be right justified
    pub fn text<'k, 's>(&'k self, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        FmtText::from(self, src, width)
    }
    /// return a formatted text, with lines wrapped or justified for the current terminal
    /// width
    /// Code blocs will be right justified
    pub fn term_text<'k, 's>(&'k self, src: &'s str) -> FmtText<'k, 's> {
        let (width, _) = Terminal::new().terminal_size();
        FmtText::from(self, src, Some(width as usize))
    }
    //pub fn text<'s, 'l>(&'s self, src: &'l str) -> FormattedText<'s, 'l> {
    //    let mut text = FormattedText::new(self, src);
    //    text.right_pad_code_blocks();
    //    tbl::fix_all_tables(&mut text, std::usize::MAX);
    //    text
    //}
    // return a formatted text adjusted for the current width of the terminal
    // Lines will be wrapped to fit in the width of the area
    // Code blocs will be right justified.
    // FIXME remove
    //pub fn wrapped_text<'s, 'l>(&'s self, src: &'l str, width: usize) -> FormattedText<'s, 'l> {
    //    let width = width + 1; // FIXME there's something to clarify here...
    //    let text = Text::from(src);
    //    let mut text = FormattedText{
    //        skin: self,
    //        text: wrap::hard_wrap_text(&text, width),
    //    };
    //    text.right_pad_code_blocks();
    //    tbl::fix_all_tables(&mut text, width);
    //    text
    //}
    // return a formatted text adjusted for the current width of the terminal
    // Lines will be wrapped to fit in the width of the area
    // Code blocs will be right justified.
    // No space is left for a scrollbar. Use area_wrapped_text
    // if you want to use the text as scrollable in a raw terminal
    // FIXME remove
    //pub fn terminal_wrapped_text<'s, 'l>(&'s self, src: &'l str) -> FormattedText<'s, 'l> {
    //    let (width, _) = Terminal::new().terminal_size();
    //    self.wrapped_text(src, width as usize)
    //}
    // return a formatted text adjusted for a specific area width.
    // Lines will be wrapped to fit in the width of the area (with
    //  the space for the scrollbar)
    // Code blocs will be right justified.
    // FIXME remove
    //pub fn area_wrapped_text<'s, 'l>(&'s self, src: &'l str, area: &Area) -> FormattedText<'s, 'l> {
    //    self.wrapped_text(src, area.width as usize - 1)
    //}
    pub fn print_inline(&self, src: &str) {
        print!("{}", self.inline(src));
    }
    pub fn print_text(&self, src: &str) {
        println!("{}", self.term_text(src));
    }

    pub fn write_fmt_composite(&self, f: &mut fmt::Formatter, fc: &FmtComposite) -> fmt::Result {
        let os = self.composite_object_style(&fc.composite.style);
        let (lp, rp) = fc.completions();
        let space = os.apply_to(" ");
        for i in 0..lp {
            write!(f, "{}", &space)?;
        }
        if fc.composite.is_list_item() {
            write!(f, "• ")?;
        }
        for c in &fc.composite.compounds {
            let os = self.compound_object_style(os, c);
            write!(f, "{}", os.apply_to(c.as_str()))?;
        }
        for i in 0..rp {
            write!(f, "{}", &space)?;
        }
        Ok(())
    }
    pub fn write_fmt_line(&self, f: &mut fmt::Formatter, line: &FmtLine) -> fmt::Result {
        match line {
            FmtLine::Normal(fc) => {
                self.write_fmt_composite(f, fc)?;
                writeln!(f)?;
            }
            FmtLine::TableRow(FmtTableRow{cells}) => {
                // FIXME tablerow
                //for composite in cells {
                //    write!(f, "│")?;
                //    let os = self.composite_object_style(&composite.style);
                //    if composite.is_list_item() {
                //        write!(f, "• ")?;
                //    }
                //    for c in &composite.compounds {
                //        let os = self.compound_object_style(os, c);
                //        write!(f, "{}", os.apply_to(c.as_str()))?;
                //    }
                //}
                writeln!(f, "│")?;
            }
            _ => {
                // FIXME table rule
            }
        }
        Ok(())
    }

    // FIXME remove
    // pub fn fmt_composite(&self, f: &mut fmt::Formatter, composite: &Composite) -> fmt::Result {
    //     let os = self.composite_object_style(&composite.style);
    //     if composite.is_list_item() {
    //         write!(f, "• ")?;
    //     }
    //     for c in &composite.compounds {
    //         let os = self.compound_object_style(os, c);
    //         write!(f, "{}", os.apply_to(c.as_str()))?;
    //     }
    //     Ok(())
    // }
    // FIXME remove
    // pub fn fmt_line(&self, f: &mut fmt::Formatter, line: &Line) -> fmt::Result {
    //     match line {
    //         Line::Normal(composite) => self.fmt_composite(f, composite),
    //         Line::TableRow(TableRow{cells}) => {
    //             for composite in cells {
    //                 write!(f, "│")?;
    //                 let os = self.composite_object_style(&composite.style);
    //                 if composite.is_list_item() {
    //                     write!(f, "• ")?;
    //                 }
    //                 for c in &composite.compounds {
    //                     let os = self.compound_object_style(os, c);
    //                     write!(f, "{}", os.apply_to(c.as_str()))?;
    //                 }
    //             }
    //             write!(f, "│")?;
    //             Ok(())
    //         }
    //     }
    // }
}

