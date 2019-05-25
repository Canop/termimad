
use crate::text::FormattedText;
use minimad::Line;

/// a sequence of lines whose line-style is Code
pub struct CodeBlock {
    pub start: usize,
    pub height: usize, // number of lines
    pub width: usize,  // length in chars of the widest line
}
impl CodeBlock {
    pub fn right_pad(&self, text: &mut FormattedText) {
        for idx in self.start..self.start + self.height {
            let line = &mut text.text.lines[idx];
            if let Line::Normal( composite ) = line {
                let len = composite.char_length();
                if len < self.width {
                    composite.pad_right(self.width - len);
                }
            }
        }
    }
}

pub fn find_blocks(text: &FormattedText) -> Vec<CodeBlock> {
    let mut blocks: Vec<CodeBlock> = Vec::new();
    let mut current: Option<CodeBlock> = None;
    for (idx, line) in text.text.lines.iter().enumerate() {
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
        } else if let Some(c) = current.take() {
            blocks.push(c);
        }
    }
    if let Some(c) = current.take() {
        blocks.push(c);
    }
    blocks
}

