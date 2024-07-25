use {
    lazy_regex::*,
    minimad::Alignment,
    std::fmt,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseAlignError {
    #[error("not a valid alignment")]
    Unrecognized,
}

pub fn write_align(f: &mut fmt::Formatter<'_>, a: Alignment) -> fmt::Result {
    match a {
        Alignment::Left => write!(f, "left"),
        Alignment::Center => write!(f, "center"),
        Alignment::Right => write!(f, "right"),
        Alignment::Unspecified => Ok(()),
    }
}

/// Read a Minimad Alignment from a string.
pub fn parse_align(s: &str) -> Result<Alignment, ParseAlignError> {
    if regex_is_match!("left"i, s) {
        Ok(Alignment::Left)
    } else if regex_is_match!("center"i, s) {
        Ok(Alignment::Center)
    } else if regex_is_match!("right"i, s) {
        Ok(Alignment::Right)
    } else {
        Err(ParseAlignError::Unrecognized)
    }
}
