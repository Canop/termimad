use {
    crate::{
        ansi,
        gray,
        rgb,
    },
    crossterm::style::Color,
    lazy_regex::*,
};

#[derive(thiserror::Error, Debug)]
pub enum ParseColorError {
    #[error("'not a recognized color")]
    Unrecognized,
    #[error("grey level must be between 0 and 23 (got {level})")]
    InvalidGreyLevel { level: u8 },
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
    regex_switch!(s,
        r"^ansi\((?<value>\d+)\)$"i => {
            let value = value.parse().map_err(|_| ParseColorError::Unrecognized)?;
            ansi(value)
        }
        r"^gr[ae]y(?:scale)?\((?<level>\d+)\)$"i => {
            let level = level.parse().map_err(|_| ParseColorError::Unrecognized)?;
            if level > 23 {
                return Err(ParseColorError::InvalidGreyLevel { level });
            }
            gray(level)
        }
        r"^rgb\((?<r>\d+),\s*(?<g>\d+),\s*(?<b>\d+)\)$"i => {
            let (Ok(r), Ok(g), Ok(b)) = (r.parse(), g.parse(), b.parse()) else {
                return Err(ParseColorError::Unrecognized);
            };
            rgb(r, g, b)
        }
        r"^#(?<r>[\da-f]{1,2})(?<g>[\da-f]{1,2})(?<b>[\da-f]{1,2})$"i => {
            let (Ok(r), Ok(g), Ok(b)) = (hex(r), hex(g), hex(b)) else {
                return Err(ParseColorError::Unrecognized);
            };
            rgb(r, g, b)
        }
        "black"i => Color::AnsiValue(16),
        "blue"i => Color::Blue,
        "cyan"i => Color::Cyan,
        "darkblue"i => Color::DarkBlue,
        "darkcyan"i => Color::DarkCyan,
        "darkgreen"i => Color::DarkGreen,
        "darkmagenta"i => Color::DarkMagenta,
        "darkred"i => Color::DarkRed,
        "green"i => Color::Green,
        "grey"i => Color::Grey,
        "magenta"i => Color::Magenta,
        "red"i => Color::Red,
        "yellow"i => Color::Yellow,
        "darkyellow"i => Color::DarkYellow,
        "white"i => Color::AnsiValue(231),
    ).ok_or(ParseColorError::Unrecognized)
}

#[test]
fn test_parse_color() {
    assert_eq!(
        parse_color("rgb(255, 35, 45)").unwrap(),
        rgb(255, 35, 45),
    );
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
    assert_eq!(parse_color("Green").unwrap(), Color::Green);
    assert_eq!(parse_color("ansi(11)").unwrap(), Color::AnsiValue(11));
}

