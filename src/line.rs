use std::fmt;

use minimad::Line;

use crate::skin::MadSkin;

pub struct FormattedLine<'s, 'l> {
    pub skin: &'s MadSkin,
    pub line: Line<'l>,
}

impl<'s, 'l> FormattedLine<'s, 'l> {
    pub fn new(skin: &'s MadSkin, text: &'l str) -> FormattedLine<'s, 'l> {
        FormattedLine {
            skin,
            line: Line::from(text),
        }
    }
}

impl fmt::Display for FormattedLine<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.skin.fmt_line(f, &self.line)
    }
}
