use std::fmt;

use minimad::Text;

use crate::skin::MadSkin;
use crate::code;

/// a formatted text, implementing Display
pub struct FormattedText<'s, 't> {
    pub skin: &'s MadSkin,
    pub text: Text<'t>,
}

impl<'s, 't> FormattedText<'s, 't> {
    pub fn new(skin: &'s MadSkin, text: &'t str) -> FormattedText<'s, 't> {
        FormattedText {
            skin,
            text: Text::from(text),
        }
    }
    pub fn right_pad_code_blocks(&mut self) {
        for b in code::find_blocks(self) {
            b.right_pad(self);
        }
    }
}

impl fmt::Display for FormattedText<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.text.lines {
            self.skin.fmt_line(f, line)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
