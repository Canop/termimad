use crate::line::FormattedLine;
use crate::area::Area;
use crate::text::FormattedText;
use crossterm::{Attribute, Color, ObjectStyle, StyledObject};
use minimad::{Compound, Line, LineStyle, MAX_HEADER_DEPTH, Text};
use std::fmt;

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
    pub normal: ObjectStyle,
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
            normal: ObjectStyle::new(),
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
    // return a formatted text.
    // Lines will be wrapped to fit in the width of the area (with
    //  the space for the scrollbar)
    // Code blocs will be right justified
    pub fn wrapped_text<'s, 'l>(&'s self, src: &'l str, area: &Area) -> FormattedText<'s, 'l> {
        let text = Text::from(src);
        let mut text = FormattedText{
            skin: self,
            text: hard_wrap_text(&text, (area.width) as usize),
        };
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
    pub fn fmt_line(&self, f: &mut fmt::Formatter, line: &Line) -> fmt::Result {
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

fn follow_up_line<'s>(line: &Line<'s>) -> Line<'s> {
    Line {
        style: match line.style {
            LineStyle::ListItem => LineStyle::Normal,
            _ => line.style,
        },
        compounds: Vec::new(),
    }
}

fn hard_wrap_line<'s>(src_line: &Line<'s>, width: usize) -> Vec<Line<'s>> {
    assert!(width > 4);
    let mut lines = Vec::new();
    let mut dst_line = Line {
        style: src_line.style,
        compounds: Vec::new(),
    };
    let mut ll = match src_line.style {
        LineStyle::ListItem => 2, // space of the puce
        _ => 0,
    };
    for sc in &src_line.compounds {
        let s = sc.as_str();
        let cl = s.chars().count();
        if ll + cl <= width {
            // we add the compound as is to the current line
            dst_line.compounds.push(sc.clone());
            ll += cl;
            continue;
        }
        if ll + 2 >= width {
            // we close the current line
            let new_dst_line = follow_up_line(&dst_line);
            lines.push(dst_line);
            dst_line = new_dst_line;
            ll = 0;
        }
        let mut c_start = 0;
        for (idx, _char) in s.char_indices() {
            ll += 1;
            if ll == width {
                dst_line.compounds.push(sc.sub(c_start, idx));
                let new_dst_line = follow_up_line(&dst_line);
                lines.push(dst_line);
                dst_line = new_dst_line;
                c_start = idx;
                ll = 0;
            }
        }
        if ll!=width {
            dst_line.compounds.push(sc.tail(c_start));
        }
    }
    lines.push(dst_line);
    lines
}
fn hard_wrap_text<'s>(text: &Text<'s>, width: usize) -> Text<'s> {
    assert!(width > 4);
    let mut lines = Vec::new();
    for src_line in &text.lines {
        for line in hard_wrap_line(src_line, width) {
            lines.push(line)
        }
    }
    Text {
        lines
    }
}

#[test]
pub fn hard_wrap() {
    // build a text and check it
    let src = "This is a *long* line which needs to be **broken**.\n\
        And the text goes on with a list:\n\
        * short item\n\
        * a *somewhat longer item* (with a part in **bold**)";
    println!("input text:\n{}", &src);
    let src_text =  Text::from(&src);
    assert_eq!(src_text.lines[0].char_length(), 45);
    assert_eq!(src_text.lines[1].char_length(), 33);
    assert_eq!(src_text.lines[2].char_length(), 10);
    assert_eq!(src_text.lines[3].char_length(), 44);

    // checking several wrapping widths
    let text = hard_wrap_text(&src_text, 100);
    assert_eq!(text.lines.len(), 4);
    let text = hard_wrap_text(&src_text, 30);
    assert_eq!(text.lines.len(), 7);
    let text = hard_wrap_text(&src_text, 12);
    assert_eq!(text.lines.len(), 12);
}

