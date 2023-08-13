use {
    crate::{
        color::*,
        styled_char::StyledChar,
    },
    crossterm::style::Color,
};

/// A scrollbar style defined by two styled chars, one
///  for the track, and one for the thumb.
///
/// For the default styling only the fg color is defined
///  and the char is ▐ but everything can be changed.
#[derive(Clone, Debug)]
pub struct ScrollBarStyle {
    pub track: StyledChar,
    pub thumb: StyledChar,
}

impl ScrollBarStyle {
    pub fn new() -> Self {
        let char = '▐';
        Self {
            track: StyledChar::from_fg_char(gray(5), char),
            thumb: StyledChar::from_fg_char(gray(21), char),
        }
    }
    pub fn set_bg(&mut self, bg: Color) {
        self.track.set_bg(bg);
        self.thumb.set_bg(bg);
    }
}

impl Default for ScrollBarStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StyledChar> for ScrollBarStyle {
    fn from(sc: StyledChar) -> Self {
        let char = sc.nude_char();
        Self {
            track: StyledChar::from_fg_char(
                sc.get_bg().unwrap_or(gray(5)),
                char,
            ),
            thumb: StyledChar::from_fg_char(
                sc.get_fg().unwrap_or(gray(21)),
                char,
            ),
        }
    }
}
