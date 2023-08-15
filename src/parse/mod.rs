//! The parse module provides a set of functions helping
//! parse colors, compound styles, line styles, from strings
mod parse_align;
mod parse_attribute;
mod parse_color;
mod parse_compound_style;
mod parse_line_style;
mod parse_styled_char;

pub use {
    parse_align::*,
    parse_attribute::*,
    parse_color::*,
    parse_compound_style::*,
    parse_line_style::*,
    parse_styled_char::*,
};

use {
    crossterm::style::{
        Attribute,
        Color,
    },
    minimad::Alignment,
    lazy_regex::*,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseStyleTokenError {
    #[error("{0} not recognized as a style token")]
    Unrecognized(String),
    #[error("Invalid color: {0}")]
    InvalidColor(#[from] ParseColorError),
}

/// something which may be part of a style
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleToken {
    Char(char),
    Color(Color),
    Attribute(Attribute),
    Align(Alignment),
    /// A specified absence, meaning for example "no foreground"
    None,
}

pub fn parse_style_token(s: &str) -> Result<StyleToken, ParseStyleTokenError> {
    if regex_is_match!("none"i, s) {
        return Ok(StyleToken::None);
    }
    match parse_color(s) {
        Ok(color) => { return Ok(StyleToken::Color(color)); }
        Err(ParseColorError::Unrecognized) => {}
        Err(e) => { return Err(e.into()); }
    }
    if let Ok(attribute) = parse_attribute(s) {
        return Ok(StyleToken::Attribute(attribute));
    }
    if let Ok(align) = parse_align(s) {
        return Ok(StyleToken::Align(align));
    }
    let mut chars = s.chars();
    let c = chars.next();
    if let Some(c) = c {
        if chars.next().is_none() {
            return Ok(StyleToken::Char(c));
        }
    }
    Err(ParseStyleTokenError::Unrecognized(s.to_owned()))
}

pub fn parse_style_tokens(s: &str) -> Result<Vec<StyleToken>, ParseStyleTokenError> {
    let mut tokens = Vec::new();
    for m in regex!(r#"[^\s()]+(\([\w,\s]+\))?"#).find_iter(s) {
        tokens.push(parse_style_token(m.as_str())?);
    }
    Ok(tokens)
}

#[test]
fn test_parse_style_tokens() {
    use {
        crate::{gray, rgb},
        StyleToken as T,
        ParseStyleTokenError as E,
        crossterm::style::Attribute::*,
        minimad::Alignment::*,
    };
    assert_eq!(
        parse_style_tokens("red bold left").unwrap(),
        vec![T::Color(Color::Red), T::Attribute(Bold), T::Align(Left)],
    );
    assert!(parse_style_tokens("red pissenlit").is_err());
    assert_eq!(
        parse_style_tokens("Center grey(15) RGB(51, 47, 58) bold").unwrap(),
        vec![T::Align(Center), T::Color(gray(15)), T::Color(rgb(51, 47, 58)), T::Attribute(Bold)],
    );
    assert_eq!(
        parse_style_tokens(" Yellow Italic ").unwrap(),
        vec![T::Color(Color::Yellow), T::Attribute(Italic)],
    );
    assert_eq!(
        parse_style_tokens("| Yellow red").unwrap(),
        vec![T::Char('|'), T::Color(Color::Yellow), T::Color(Color::Red)],
    );
    assert_eq!(
        parse_style_tokens("rgb(255,0,100) #fb0").unwrap(),
        vec![T::Color(rgb(255,0,100)), T::Color(rgb(255,187,0))],
    );
    let parsed = parse_style_tokens(" red gray(40) ");
    if let Err(E::InvalidColor(ParseColorError::InvalidGreyLevel{level})) = parsed {
        assert_eq!(level, 40);
    } else {
        panic!("failed to fail");
    };
}
