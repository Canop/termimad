use {
    crate::{
        code,
        composite_kind::CompositeKind,
        fit::wrap,
        line::FmtLine,
        skin::MadSkin,
        tbl,
    },
    minimad::{
        Line,
        parse_text,
        Options,
        Text,
    },
    std::fmt,
};

/// a formatted text, implementing Display.
///
/// The text is wrapped for the width given at build, which
/// means the rendering height is the number of lines.
///
/// ```
/// use termimad::*;
/// let skin = MadSkin::default();
/// let my_markdown = "#title\n* item 1\n* item 2";
/// let text = FmtText::from(&skin, &my_markdown, Some(80));
/// println!("{}", &text);
/// ```
#[derive(Debug)]
pub struct FmtText<'k, 's> {
    pub skin: &'k MadSkin,
    pub lines: Vec<FmtLine<'s>>,
    pub width: Option<usize>, // available width
}

impl<'k, 's> FmtText<'k, 's> {
    /// build a displayable text for the specified width and skin
    ///
    /// This can be called directly or using one of the skin helper
    /// method.
    pub fn from(skin: &'k MadSkin, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        let mt = parse_text(src, Options::default().keep_code_fences(true));
        Self::from_text(skin, mt, width)
    }
    /// build a text as raw (with no markdown interpretation)
    pub fn raw_str(skin: &'k MadSkin, src: &'s str, width: Option<usize>) -> FmtText<'k, 's> {
        let mt = Text::raw_str(src);
        Self::from_text(skin, mt, width)
    }

    /// build a fmt_text from a minimad text
    pub fn from_text(
        skin: &'k MadSkin,
        mut text: Text<'s>,
        width: Option<usize>,
    ) -> FmtText<'k, 's> {
        // Manual loop (instead of drain().map()) to track the current code-fence language tag.
        let mut lines: Vec<FmtLine<'s>> = Vec::with_capacity(text.lines.len());
        let mut current_code_lang: Option<String> = None;
        for mline in text.lines.drain(..) {
            match mline {
                Line::CodeFence(ref composite) => {
                    // Opening fence carries the language; closing fence has no compounds → None.
                    current_code_lang = composite
                        .compounds
                        .first()
                        .map(|c| c.src.to_string())
                        .filter(|s| !s.is_empty());
                    // CodeFence lines are not pushed to the output.
                }
                other => {
                    let mut fmt_line = FmtLine::from(other, skin);
                    // Attach the language tag to Code composites.
                    if let FmtLine::Normal(ref mut fc) = fmt_line {
                        if fc.kind == CompositeKind::Code {
                            fc.code_lang = current_code_lang.clone();
                        }
                    }
                    lines.push(fmt_line);
                }
            }
        }
        tbl::fix_all_tables(&mut lines, width.unwrap_or(usize::MAX), skin);
        code::justify_blocks(&mut lines);
        // Syntax highlighting replaces Normal(Code) lines with HighlightedCode lines.
        // This must happen after justify_blocks so block widths are already set.
        code::highlight_blocks(&mut lines, skin);
        if let Some(width) = width {
            if width >= 3 {
                lines =
                    wrap::hard_wrap_lines(lines, width, skin).expect("width should be wide enough");
            }
        }
        FmtText { skin, lines, width }
    }
    /// set the width to render the text to.
    ///
    /// It's preferable to set it no smaller than content_width and
    /// no wider than the terminal's width.
    ///
    /// If you want the text to be wrapped, pass a width on construction
    /// (ie in FmtText::from or FmtText::from_text) instead.
    /// The main purpose of this function is to optimize the rendering
    /// of a text (or several ones) to a content width, for example to
    /// have centered titles centered not based on the terminal's width
    /// but on the content width
    pub fn set_rendering_width(&mut self, width: usize) {
        self.width = Some(width);
    }
    pub fn content_width(&self) -> usize {
        self.lines
            .iter()
            .fold(0, |cw, line| cw.max(line.visible_length()))
    }
}

impl fmt::Display for FmtText<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            self.skin.write_fmt_line(f, line, self.width, false)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
