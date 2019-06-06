use std::fmt;

use minimad::Text;

use crate::skin::MadSkin;
use crate::code;
use crate::line::FmtLine;
use crate::tbl;
use crate::wrap;

/// a formatted text, implementing Display
pub struct FmtText<'k, 's> {
    pub skin: &'k MadSkin,
    pub lines: Vec<FmtLine<'s>>,
    pub width: Option<usize>, // available width
}

impl<'k, 's> FmtText<'k, 's> {
    pub fn from(skin: &'k MadSkin, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        let mut mt = Text::from(src);
        let mut lines = mt.lines.drain(..).map(
            |mline| FmtLine::from(mline, skin)
        ).collect();

        tbl::fix_all_tables(&mut lines, width.unwrap_or(std::usize::MAX));
        code::justify_blocks(&mut lines);
        if let Some(width) = width {
            lines = wrap::hard_wrap_lines(lines, width);
        }

        FmtText {
            skin,
            lines,
            width,
        }
    }
}

impl fmt::Display for FmtText<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            self.skin.write_fmt_line(f, line, self.width)?;
        }
        Ok(())
    }
}

