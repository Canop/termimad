use {
    crossterm::style::{
        Attribute,
    },
    lazy_regex::*,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseAttributeError {
    #[error("not a recognized attribute")]
    Unrecognized,
}

/// Read a Minimad Attributement from a string.
pub fn parse_attribute(s: &str) -> Result<Attribute, ParseAttributeError> {
    if regex_is_match!("bold"i, s) {
        Ok(Attribute::Bold)
    } else if regex_is_match!("crossed[_-]?out"i, s) {
        Ok(Attribute::CrossedOut)
    } else if regex_is_match!("dim"i, s) {
        Ok(Attribute::Dim)
    } else if regex_is_match!("italic"i, s) {
        Ok(Attribute::Italic)
    } else if regex_is_match!("under[_-]?lined"i, s) {
        Ok(Attribute::Underlined)
    } else if regex_is_match!("over[_-]?lined"i, s) {
        Ok(Attribute::OverLined)
    } else if regex_is_match!("reverse"i, s) {
        Ok(Attribute::Reverse)
    } else if regex_is_match!("encircled"i, s) {
        Ok(Attribute::Encircled)
    } else if regex_is_match!("slow[_-]?blink"i, s) {
        Ok(Attribute::SlowBlink)
    } else if regex_is_match!("rapid[_-]?blink"i, s) {
        Ok(Attribute::RapidBlink)
    } else {
        Err(ParseAttributeError::Unrecognized)
    }
}

