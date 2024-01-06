use {
    crate::compound_style::CompoundStyle,
    crossterm::style::{Attribute, Color},
    minimad::Alignment,
    std::fmt,
};

/// A style applicable to a type of line.
///
/// It's made of
///  - the base style of the compounds
///  - the alignment
#[derive(Default, Clone, Debug, PartialEq)]
pub struct LineStyle {
    pub compound_style: CompoundStyle,
    pub align: Alignment,
    pub left_margin: usize,
    pub right_margin: usize,
}

impl LineStyle {

    /// Return a (left_margin, right_margin) tupple, with both values
    /// being zeroed when they wouldn't let a width of at least 3 otherwise.
    pub fn margins_in(&self, available_width: Option<usize>) -> (usize, usize) {
        if let Some(width) = available_width {
            if width < self.left_margin + self.right_margin + 3 {
                return (0, 0);
            }
        }
        (self.left_margin, self.right_margin)
    }

    pub fn new(
        compound_style: CompoundStyle,
        align: Alignment,
    ) -> Self {
        Self {
            compound_style,
            align,
            left_margin: 0,
            right_margin: 0,
        }
    }

    /// Set the foreground color to the passed color.
    #[inline(always)]
    pub fn set_fg(&mut self, color: Color) {
        self.compound_style.set_fg(color);
    }

    /// Set the background color to the passed color.
    #[inline(always)]
    pub fn set_bg(&mut self, color: Color) {
        self.compound_style.set_bg(color);
    }

    /// Set the colors to the passed ones
    pub fn set_fgbg(&mut self, fg: Color, bg: Color) {
        self.compound_style.set_fgbg(fg, bg);
    }

    /// Add an `Attribute`. Like italic, underlined or bold.
    #[inline(always)]
    pub fn add_attr(&mut self, attr: Attribute) {
        self.compound_style.add_attr(attr);
    }

    /// Write a string several times with the line compound style
    #[inline(always)]
    pub fn repeat_string(&self, f: &mut fmt::Formatter<'_>, s: &str, count: usize) -> fmt::Result {
        self.compound_style.repeat_string(f, s, count)
    }

    /// Write a string several times with the line compound style
    #[inline(always)]
    pub fn repeat_char(&self, f: &mut fmt::Formatter<'_>, c: char, count: usize) -> fmt::Result {
        self.compound_style.repeat_char(f, c, count)
    }

    /// Write 0 or more spaces with the line's compound style
    #[inline(always)]
    pub fn repeat_space(&self, f: &mut fmt::Formatter<'_>, count: usize) -> fmt::Result {
        self.repeat_char(f, ' ', count)
    }

    pub fn blend_with<C: Into<coolor::Color>>(&mut self, color: C, weight: f32) {
        self.compound_style.blend_with(color, weight);
    }
}

impl From<CompoundStyle> for LineStyle {
    fn from(compound_style: CompoundStyle) -> Self {
        Self {
            compound_style,
            align: Alignment::Unspecified,
            left_margin: 0,
            right_margin: 0,
        }
    }
}
