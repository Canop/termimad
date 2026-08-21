//! Tests of the parsing of colors and style tokens

use termimad::{
    crossterm::style::Color,
    parse_color,
    parse_style_tokens,
    rgb,
    ParseColorError,
};

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
    assert_eq!(parse_color("Green").unwrap(), Color::Green);
    assert_eq!(parse_color("ansi(11)").unwrap(), Color::AnsiValue(11));
}

#[test]
fn test_parse_style_tokens() {
    use {
        termimad::{
            crossterm::style::Attribute::*,
            gray,
            rgb,
        },
        termimad::minimad::Alignment::*,
        termimad::ParseStyleTokenError as E,
        termimad::StyleToken as T,
    };
    assert_eq!(
        parse_style_tokens("red bold left").unwrap(),
        vec![T::Color(Color::Red), T::Attribute(Bold), T::Align(Left)],
    );
    assert!(parse_style_tokens("red pissenlit").is_err());
    assert_eq!(
        parse_style_tokens("Center grey(15) RGB(51, 47, 58) bold").unwrap(),
        vec![
            T::Align(Center),
            T::Color(gray(15)),
            T::Color(rgb(51, 47, 58)),
            T::Attribute(Bold)
        ],
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
        vec![T::Color(rgb(255, 0, 100)), T::Color(rgb(255, 187, 0))],
    );
    let parsed = parse_style_tokens(" red gray(40) ");
    if let Err(E::InvalidColor(ParseColorError::InvalidGreyLevel { level })) = parsed {
        assert_eq!(level, 40);
    } else {
        panic!("failed to fail");
    };
}
