use {
    crossterm::style::{
        Attribute,
    },
    lazy_regex::*,
    std::fmt,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseAttributeError {
    #[error("not a recognized attribute")]
    Unrecognized,
}

pub fn write_attribute(f: &mut fmt::Formatter<'_>, a: Attribute) -> fmt::Result {
    match a {
        Attribute::Reset => Ok(()),
        Attribute::Bold => write!(f, "Bold"),
        Attribute::Dim => write!(f, "Dim"),
        Attribute::Italic => write!(f, "Italic"),
        Attribute::Underlined => write!(f, "Underlined"),
        Attribute::SlowBlink => write!(f, "SlowBlink"),
        Attribute::RapidBlink => write!(f, "RapidBlink"),
        Attribute::Reverse => write!(f, "Reverse"),
        Attribute::Hidden => write!(f, "Hidden"),
        Attribute::CrossedOut => write!(f, "CrossedOut"),
        Attribute::Fraktur => write!(f, "Fraktur"),
        Attribute::NoBold => write!(f, "NoBold"),
        Attribute::NormalIntensity => write!(f, "NormalIntensity"),
        Attribute::NoItalic => write!(f, "NoItalic"),
        Attribute::NoUnderline => write!(f, "NoUnderline"),
        Attribute::NoBlink => write!(f, "NoBlink"),
        Attribute::NoReverse => write!(f, "NoReverse"),
        Attribute::NoHidden => write!(f, "NoHidden"),
        Attribute::NotCrossedOut => write!(f, "NotCrossedOut"),
        Attribute::Framed => write!(f, "Framed"),
        Attribute::Encircled => write!(f, "Encircled"),
        Attribute::OverLined => write!(f, "OverLined"),
        Attribute::NotFramedOrEncircled => write!(f, "NotFramedOrEncircled"),
        Attribute::NotOverLined => write!(f, "NotOverLined"),
        _ => Ok(()),
    }
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

