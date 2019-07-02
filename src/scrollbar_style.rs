
use crate::color::*;
use crate::styled_char::StyledChar;

/// The scrollbar style is defined by two styled chars, one
///  for the track, and one for the thumb.
/// For the default styling only the fg color is defined
///  and the char is ▐ but everything can be changed
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
}
