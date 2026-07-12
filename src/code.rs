use {
    crate::*,
    minimad::Alignment,
};

/// a sequence of lines whose line-style is Code
#[derive(Debug)]
pub struct CodeBlock {
    pub start: usize,
    pub height: usize, // number of lines
    pub width: usize,  // length in chars of the widest line
}
impl CodeBlock {
    /// ensure all lines of the block have the same width
    pub fn justify(&self, lines: &mut [FmtLine<'_>]) {
        for line in lines.iter_mut().skip(self.start).take(self.height) {
            if let FmtLine::Normal(ref mut fc) = line {
                fc.spacing = Some(Spacing {
                    width: self.width,
                    align: Alignment::Left,
                });
            }
        }
    }
}

const fn code_line_length(line: &FmtLine<'_>) -> Option<usize> {
    match line {
        FmtLine::Normal(fc) => match fc.kind {
            CompositeKind::Code => Some(fc.visible_length),
            _ => None,
        },
        _ => None,
    }
}

/// find ranges of code lines in a text.
///
/// Warning: the indices in a codeblock are invalid as
/// soon as lines are inserted or removed. This function
/// should normally not be used from another module or lib
pub fn find_blocks(lines: &[FmtLine<'_>]) -> Vec<CodeBlock> {
    let mut blocks: Vec<CodeBlock> = Vec::new();
    let mut current: Option<CodeBlock> = None;
    for (idx, line) in lines.iter().enumerate() {
        if let Some(ll) = code_line_length(line) {
            match current.as_mut() {
                Some(b) => {
                    b.height += 1;
                    b.width = b.width.max(ll);
                }
                None => {
                    current = Some(CodeBlock {
                        start: idx,
                        height: 1,
                        width: ll,
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

/// ensure the widths of all lines in a code block are
/// the same line.
pub fn justify_blocks(lines: &mut [FmtLine<'_>]) {
    let blocks = find_blocks(lines);
    for b in blocks {
        b.justify(lines);
    }
}

/// If the skin has a `code_syntax_highlighter`, replace all `FmtLine::Normal(Code)` blocks
/// with `FmtLine::HighlightedCode` lines.
///
/// This must be called *after* `justify_blocks` so that block widths are already set.
pub fn highlight_blocks(lines: &mut Vec<FmtLine<'_>>, skin: &MadSkin) {
    let Some(ref highlighter) = skin.code_syntax_highlighter else {
        return;
    };

    let blocks = find_blocks(lines);
    // Process in reverse so that splice indices remain valid.
    for block in blocks.into_iter().rev() {
        // Collect source text and metadata from the block's lines.
        let mut lang: Option<String> = None;
        let mut block_width: usize = 0;
        let code_lines: Vec<String> = lines
            .iter()
            .skip(block.start)
            .take(block.height)
            .filter_map(|l| {
                if let FmtLine::Normal(fc) = l {
                    if lang.is_none() {
                        lang = fc.code_lang.clone();
                    }
                    block_width = block_width
                        .max(fc.spacing.map(|s| s.width).unwrap_or(fc.visible_length));
                    Some(fc.compounds.iter().map(|c| c.src).collect::<String>())
                } else {
                    None
                }
            })
            .collect();

        if code_lines.is_empty() {
            continue;
        }

        let highlighted = highlighter.highlight(&code_lines.join("\n"), lang.as_deref());

        let new_lines: Vec<FmtLine<'_>> = highlighted
            .into_iter()
            .zip(code_lines.iter())
            .map(|(ansi_line, raw_line)| {
                FmtLine::HighlightedCode(HighlightedCodeLine {
                    content: ansi_line,
                    visible_len: raw_line.chars().count(),
                    block_width,
                })
            })
            .collect();

        lines.splice(block.start..block.start + block.height, new_lines);
    }
}
