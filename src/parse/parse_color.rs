use {
    crate::{
        ansi,
        crossterm::style::Color,
        gray,
        rgb,
    },
    lazy_regex::*,
    std::fmt,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseColorError {
    #[error("'not a recognized color")]
    Unrecognized,
    #[error("grey level must be between 0 and 23 (got {level})")]
    InvalidGreyLevel { level: u8 },
}

pub fn write_color(f: &mut fmt::Formatter<'_>, c: Color) -> fmt::Result {
    match c {
        Color::Reset => Ok(()),
        Color::Black => write!(f, "Black"),
        Color::DarkGrey => write!(f, "DarkGrey"),
        Color::Red => write!(f, "Red"),
        Color::DarkRed => write!(f, "DarkRed"),
        Color::Green => write!(f, "Green"),
        Color::DarkGreen => write!(f, "DarkGreen"),
        Color::Yellow => write!(f, "Yellow"),
        Color::DarkYellow => write!(f, "DarkYellow"),
        Color::Blue => write!(f, "Blue"),
        Color::DarkBlue => write!(f, "DarkBlue"),
        Color::Magenta => write!(f, "Magenta"),
        Color::DarkMagenta => write!(f, "DarkMagenta"),
        Color::Cyan => write!(f, "Cyan"),
        Color::DarkCyan => write!(f, "DarkCyan"),
        Color::White => write!(f, "White"),
        Color::Grey => write!(f, "Grey"),
        Color::Rgb { r, g, b } => write!(f, "rgb({r}, {g}, {b})"),
        Color::AnsiValue(code) => write!(f, "ansi({code})"),
    }
}

/// Read a Crossterm color from a string.
///
/// It may be either
/// - one of the few known color name. Example: "darkred"
/// - grayscale with level in [0,24[. Example: "grey(5)"
/// - an Ansi code. Example "ansi(106)"
/// - RGB. Example: "rgb(25, 100, 0)"
pub fn parse_color(s: &str) -> Result<Color, ParseColorError> {
    fn hex(s: &str) -> Result<u8, std::num::ParseIntError> {
        if s.len() == 1 {
            u8::from_str_radix(&format!("{s}{s}"), 16)
        } else {
            u8::from_str_radix(s, 16)
        }
    }
    if let Some((_, value)) = regex_captures!(r"^ansi\((?P<value>\d+)\)$"i, s) {
        let value = value.parse();
        if let Ok(value) = value {
            return Ok(ansi(value)); // all ANSI values are ok
        } else {
            return Err(ParseColorError::Unrecognized);
        }
    }

    if let Some((_, level)) = regex_captures!(r"^gr[ae]y(?:scale)?\((?P<level>\d+)\)$"i, s) {
        let level = level.parse();
        if let Ok(level) = level {
            if level > 23 {
                return Err(ParseColorError::InvalidGreyLevel { level });
            }
            return Ok(gray(level));
        } else {
            return Err(ParseColorError::Unrecognized);
        }
    }

    if let Some((_, r, g, b)) =
        regex_captures!(r"^rgb\((?P<r>\d+),\s*(?P<g>\d+),\s*(?P<b>\d+)\)$"i, s)
    {
        if let (Ok(r), Ok(g), Ok(b)) = (r.parse(), g.parse(), b.parse()) {
            return Ok(rgb(r, g, b));
        } else {
            return Err(ParseColorError::Unrecognized);
        }
    }

    if let Some((_, r, g, b)) =
        regex_captures!(r"^#([\da-f]{1,2})([\da-f]{1,2})([\da-f]{1,2})$"i, s)
    {
        if let (Ok(r), Ok(g), Ok(b)) = (hex(r), hex(g), hex(b)) {
            return Ok(rgb(r, g, b));
        } else {
            return Err(ParseColorError::Unrecognized);
        }
    }

    let s = s.to_lowercase();
    match s.as_str() {
        "black" => Ok(Color::AnsiValue(16)),
        "blue" => Ok(Color::Blue),
        "cyan" => Ok(Color::Cyan),
        "darkblue" => Ok(Color::DarkBlue),
        "darkcyan" => Ok(Color::DarkCyan),
        "darkgreen" => Ok(Color::DarkGreen),
        "darkmagenta" => Ok(Color::DarkMagenta),
        "darkred" => Ok(Color::DarkRed),
        "green" => Ok(Color::Green),
        "grey" => Ok(Color::Grey),
        "magenta" => Ok(Color::Magenta),
        "red" => Ok(Color::Red),
        "yellow" => Ok(Color::Yellow),
        "darkyellow" => Ok(Color::DarkYellow),
        "white" => Ok(Color::AnsiValue(231)),
        _ => Err(ParseColorError::Unrecognized),
    }
}

#[test]
fn test_parse_color() {
    assert_eq!(parse_color("rgb(255, 35, 45)").unwrap(), rgb(255, 35, 45),);
    assert!(matches!(
        parse_color("rgb(255, 260, 45)"),
        Err(ParseColorError::Unrecognized),
    ));
    assert!(matches!(
        parse_color("gray(25)"),
        Err(ParseColorError::InvalidGreyLevel { level: 25 }),
    ));
    assert_eq!(
        parse_color("gray(11)").unwrap(),
        parse_color("GREY(11)").unwrap(),
    );
}
