use std::fmt;

use minimad::Text;
use minimad::{Alignment, Line};

use crate::skin::MadSkin;
use crate::code;
use crate::line::FmtLine;
use crate::tbl::*;
use crate::wrap;


/// a formatted text, implementing Display
pub struct FmtText<'k, 's> {
    pub skin: &'k MadSkin,
    pub lines: Vec<FmtLine<'s>>,
}

impl<'k, 's> FmtText<'k, 's> {
    pub fn from(skin: &'k MadSkin, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        let mut mt = Text::from(src);
        let mut lines = mt.lines.drain(..).map(
            |mline| FmtLine::from(mline, skin)
        ).collect();


        // HERE fix tables

        code::justify_blocks(&mut lines);
        if let Some(width) = width {
            lines = wrap::hard_wrap_lines(lines, width);
        }

        FmtText {
            skin,
            lines,
        }
    }
}

impl fmt::Display for FmtText<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.lines {
            self.skin.write_fmt_line(f, line)?;
        }
        Ok(())
    }
}

