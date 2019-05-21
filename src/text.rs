use std::fmt;

use minimad::{Compound, Text};

use crate::skin::MadSkin;

static SPACES: &'static str = "                                                                ";

/// a formatted text, implementing Display
pub struct FormattedText<'s, 't> {
    pub skin: &'s MadSkin,
    pub text: Text<'t>,
}

pub struct CodeBlock {
    pub start: usize,
    pub height: usize, // number of lines
    pub width: usize,  // length in chars of the widest line
}

impl<'s, 't> FormattedText<'s, 't> {
    pub fn new(skin: &'s MadSkin, text: &'t str) -> FormattedText<'s, 't> {
        FormattedText {
            skin,
            text: Text::from(text),
        }
    }
    pub fn find_code_blocks(&self) -> Vec<CodeBlock> {
        let mut blocks: Vec<CodeBlock> = Vec::new();
        let mut current: Option<CodeBlock> = None;
        for (idx, line) in self.text.lines.iter().enumerate() {
            if line.is_code() {
                match current.as_mut() {
                    Some(b) => {
                        b.height += 1;
                        b.width = b.width.max(line.char_length());
                    }
                    None => {
                        current = Some(CodeBlock {
                            start: idx,
                            height: 1,
                            width: line.char_length(),
                        });
                    }
                }
            } else if current.is_some() {
                blocks.push(current.take().unwrap());
            }
        }
        if current.is_some() {
            blocks.push(current.take().unwrap());
        }
        blocks
    }
    pub fn right_pad_code_blocks(&mut self) {
        let max_pad = SPACES.len();
        for b in self.find_code_blocks() {
            for idx in b.start..b.start + b.height {
                let line = &mut self.text.lines[idx];
                let len = line.char_length();
                if len < b.width {
                    let pad_len = (b.width - len).min(max_pad);
                    line.compounds.push(Compound::raw_part(&SPACES, 0, pad_len));
                }
            }
        }
    }
    pub fn count_rows_per_line(&self, width: u16) -> Vec<usize> {
        self.text.lines.iter().map(
            |l| ((l.char_length() as u16 + width - 1) / width) as usize
        ).collect()
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
