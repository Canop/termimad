use std::fmt;
use minimad::Line;

use crate::skin::MadSkin;
use crate::line::FmtLine;


/// an internal facility to write just a line of a text
pub struct DisplayableLine<'s, 'l, 'p> {
    pub skin: &'s MadSkin,
    pub line: &'p FmtLine<'l>,
}

impl<'s, 'l, 'p> DisplayableLine<'s, 'l, 'p> {
    pub fn new(skin: &'s MadSkin, line: &'p FmtLine<'l>) -> DisplayableLine<'s, 'l, 'p> {
        DisplayableLine {
            skin,
            line
        }
    }
}

impl fmt::Display for DisplayableLine<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.skin.write_fmt_line(f, self.line)
    }
}
