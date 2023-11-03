use {
    super::*,
    crate::{
        StyledChar,
    },
};

/// Read a styled char from a string.
pub fn parse_styled_char(s: &str, default_nude_char: char) -> Result<StyledChar, ParseStyleTokenError> {
    let tokens = parse_style_tokens(s)?;
    let style = tokens.as_slice().into();
    let nude_char = tokens
        .iter()
        .find_map(|token| match token {
            StyleToken::Char(c) => Some(*c),
            _ => None,
        })
        .unwrap_or(default_nude_char);
    Ok(StyledChar::new(style, nude_char))
}

impl PushStyleTokens for StyledChar {
    fn push_style_tokens(&self, tokens: &mut Vec<StyleToken>) {
        tokens.push(StyleToken::Char(self.nude_char()));
        self.compound_style().push_style_tokens(tokens);
    }
}

