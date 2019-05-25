use std::fmt;
use minimad::Line;

use crate::skin::MadSkin;

pub struct DisplayableLine<'s, 'l, 'p> {
    pub skin: &'s MadSkin,
    pub line: &'p Line<'l>,
}

impl<'s, 'l, 'p> DisplayableLine<'s, 'l, 'p> {
    pub fn new(skin: &'s MadSkin, line: &'p Line<'l>) -> DisplayableLine<'s, 'l, 'p> {
        DisplayableLine {
            skin,
            line
        }
    }
}

impl fmt::Display for DisplayableLine<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.skin.fmt_line(f, self.line)
    }
}
