use {
    super::*,
    crate::{
        LineStyle,
    },
};

/// Read a Minimad CompoundStyle from a string.
pub fn parse_line_style(s: &str) -> Result<LineStyle, ParseStyleTokenError> {
    let tokens = parse_style_tokens(s)?;
    Ok(tokens.as_slice().into())
}

impl From<&[StyleToken]> for LineStyle {
    fn from(tokens: &[StyleToken]) -> Self {
        let compound_style = tokens.into();
        let align = tokens
            .iter()
            .find_map(|token| match token {
                StyleToken::Align(a) => Some(*a),
                _ => None,
            })
            .unwrap_or_default();
        LineStyle { compound_style, align }
    }
}

impl PushStyleTokens for LineStyle {
    fn push_style_tokens(&self, tokens: &mut Vec<StyleToken>) {
        self.compound_style.push_style_tokens(tokens);
        tokens.push(StyleToken::Align(self.align));
    }
}
