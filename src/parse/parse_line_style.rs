use {
    super::*,
    crate::{
        LineStyle,
    },
};

/// Read a line_style from a string.
pub fn parse_line_style(s: &str) -> Result<LineStyle, ParseStyleTokenError> {
    let tokens = parse_style_tokens(s)?;
    Ok(tokens.as_slice().into())
}

impl From<&[StyleToken]> for LineStyle {
    fn from(tokens: &[StyleToken]) -> Self {
        let compound_style = tokens.into();
        let mut left_margin = None;
        let mut right_margin = None;
        let mut align = Default::default();
        for token in tokens {
            match token {
                StyleToken::Align(a) => {
                    align = *a;
                }
                StyleToken::Dimension(number) => {
                    if left_margin.is_some() {
                        right_margin = Some(*number);
                    } else {
                        left_margin = Some(*number);
                    }
                }
                _ => {}
            }
        }
        let left_margin = left_margin.unwrap_or_default() as usize;
        let right_margin = right_margin.unwrap_or_default() as usize;
        LineStyle { compound_style, align, left_margin, right_margin }
    }
}

impl PushStyleTokens for LineStyle {
    fn push_style_tokens(&self, tokens: &mut Vec<StyleToken>) {
        self.compound_style.push_style_tokens(tokens);
        tokens.push(StyleToken::Align(self.align));
        if self.left_margin > 0 || self.right_margin > 0 {
            tokens.push(StyleToken::Dimension(self.left_margin.min(65536) as u16));
            tokens.push(StyleToken::Dimension(self.right_margin.min(65536) as u16));
        }
    }
}
