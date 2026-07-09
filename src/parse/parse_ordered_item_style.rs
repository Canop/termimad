use {
    super::*,
    crate::*,
};


impl PushStyleTokens for OrderedItemStyle {
    fn push_style_tokens(&self, tokens: &mut Vec<StyleToken>) {
        self.index_style.push_style_tokens(tokens);
        self.index_suffix.push_style_tokens(tokens);
    }
}

/// Read an ordered item style from a string.
pub fn parse_ordered_item_style(
    s: &str,
    default_suffix_char: char,
) -> Result<OrderedItemStyle, ParseStyleTokenError> {
    let tokens = parse_style_tokens(s)?;
    let char_token_idx = tokens.iter().position(|token| matches!(token, StyleToken::Char(_)));
    if let Some(idx) = char_token_idx {
        let (index_style_tokens, suffix_tokens) = tokens.split_at(idx);
        let index_style = index_style_tokens.into();
        let suffix_char = match &suffix_tokens[0] {
            StyleToken::Char(c) => *c,
            _ => default_suffix_char,
        };
        let suffix_style = suffix_tokens[1..].into();
        Ok(OrderedItemStyle {
            index_style,
            index_suffix: StyledChar::new(suffix_style, suffix_char),
        })
    } else {
        // No char token found, use default suffix char, and all the style tokens are
        // for the index style.
        let style = tokens.as_slice().into();
        Ok(OrderedItemStyle {
            index_style: style,
            index_suffix: StyledChar::nude(default_suffix_char),
        })
    }
}
