use {
    super::*,
    crate::{
        ATTRIBUTES,
        CompoundStyle,
    },
};

/// Read a Minimad CompoundStyle from a string.
///
/// May contain attributes and from 0 to 2 colors, the first encountered
/// one being the foreground.
///
/// Examples:
/// * `#ab00c1 strikeout`
/// * `"red yellow bold italic"`
/// * `""`
/// * `"gray(2) gray(20)"`
/// * `""`
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
                StyleToken::Dimension(_) => {
                    // not of use for compound styles
                }
            }
        }
        style
    }
}

impl PushStyleTokens for CompoundStyle {
    fn push_style_tokens(&self, tokens: &mut Vec<StyleToken>) {
        if let Some(fg) = self.get_fg() {
            tokens.push(StyleToken::Color(fg));
        } else if self.get_bg().is_some() {
            tokens.push(StyleToken::None);
        }
        if let Some(bg) = self.get_bg() {
            tokens.push(StyleToken::Color(bg));
        }
        for &attr in ATTRIBUTES {
            if self.has_attr(attr) {
                tokens.push(StyleToken::Attribute(attr));
            }
        }
    }
}
