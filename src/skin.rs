use crate::line::FormattedLine;
use crate::text::FormattedText;
use crossterm::{Attribute, Color, ObjectStyle};
use minimad::{Compound, Line, LineStyle, MAX_HEADER_DEPTH};
use std::fmt;

/// A skin defining how a parsed mardkown appears on the terminal
/// (fg and bg colors, bold, italic, underline, etc.)
pub struct MadSkin {
    pub normal: ObjectStyle,
    pub bold: ObjectStyle,
    pub italic: ObjectStyle,
    pub code: ObjectStyle,
    pub headers: [ObjectStyle; MAX_HEADER_DEPTH],
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
    pub fn new() -> MadSkin {
        let mut skin = MadSkin {
            normal: ObjectStyle::new(),
            bold: ObjectStyle::new(),
            italic: ObjectStyle::new(),
            code: ObjectStyle::new(),
            headers: Default::default(),
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
    fn line_object_style(&self, line_style: &LineStyle) -> &ObjectStyle {
        match line_style {
            LineStyle::Code => &self.code,
            LineStyle::Header(level) if *level <= MAX_HEADER_DEPTH as u8 => {
                &self.headers[*level as usize - 1]
            }
            _ => &self.normal,
        }
    }
    fn compound_object_style(
        &self,
        line_object_style: &ObjectStyle,
        compound: &Compound,
    ) -> ObjectStyle {
        let mut os = line_object_style.clone();
        if compound.bold {
            add_style(&mut os, &self.bold);
        }
        if compound.italic {
            add_style(&mut os, &self.italic);
        }
        if compound.code {
            add_style(&mut os, &self.code);
        }
        os
    }
    // return a formatted line
    // Don't use this function if `src` is expected to be several lines.
    pub fn line<'s, 'l>(&'s self, src: &'l str) -> FormattedLine<'s, 'l> {
        FormattedLine::new(self, src)
    }
    // return a formatted text.
    // Code blocs will be right justified
    pub fn text<'s, 'l>(&'s self, src: &'l str) -> FormattedText<'s, 'l> {
        let mut text = FormattedText::new(self, src);
        text.right_pad_code_blocks();
        text
    }
    pub fn print_line(&self, src: &str) {
        print!("{}", FormattedLine::new(self, src));
    }
    pub fn print_line_ln(&self, src: &str) {
        println!("{}", FormattedLine::new(self, src));
    }
    pub fn print_text(&self, src: &str) {
        println!("{}", FormattedText::new(self, src));
    }
    pub fn fmt_line(&self, f: &mut fmt::Formatter, indent: u8, line: &Line) -> fmt::Result {
        if indent > 0 {
            write!(f, "{}", " ".repeat(indent as usize))?;
        }
        let os = self.line_object_style(&line.style);
        if line.is_list_item() {
            write!(f, "• ")?;
        }
        for c in &line.compounds {
            let os = self.compound_object_style(os, c);
            write!(f, "{}", os.apply_to(c.as_str()))?;
        }
        Ok(())
    }
}
