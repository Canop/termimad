use {
    super::*,
    crate::CompoundStyle,
};

/// Read a Minimad CompoundStyle from a string.
pub fn parse_compound_style(s: &str) -> Result<CompoundStyle, ParseStyleTokenError> {
    let tokens = parse_style_tokens(s)?;
    Ok(tokens.as_slice().into())
}

impl From<&[StyleToken]> for CompoundStyle {
    fn from(tokens: &[StyleToken]) -> Self {
        let mut style = Self::default();
        // first encountered color or None is considered as the foreground
        // and the following one(s) as the background
        let mut fg_set = false;
        for token in tokens {
            match token {
                StyleToken::Color(c) => {
                    if fg_set {
                        style.set_bg(*c);
                    } else {
                        style.set_fg(*c);
                        fg_set = true;
                    }
                }
                StyleToken::None => {
                    if !fg_set {
                        fg_set = true;
                    }
                }
                StyleToken::Attribute(attribute) => {
                    style.add_attr(*attribute);
                }
                StyleToken::Char(_) => {
                    // not of use for compound styles
                }
                StyleToken::Align(_) => {
                    // not of use for compound styles
                }
            }
        }
        style
    }
}
