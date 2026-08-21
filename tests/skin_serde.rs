//! Tests of skin serialization and deserialization

use termimad::{
    minimad::Alignment,
    MadSkin,
};

/// Check that serializing a skin in JSON, then deserializing this
/// JSON into a new skin, results in an identical skin.
#[test]
fn skin_json_roundtrip() {
    use {
        termimad::{
            crossterm::style::{
                Attribute,
                Color::*,
            },
            gray,
            rgb,
            StyledChar,
            ROUNDED_TABLE_BORDER_CHARS,
        },
        pretty_assertions::assert_eq,
    };

    let skin = MadSkin::default();
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(skin, deserialized);

    let mut skin = MadSkin::no_style();
    skin.limit_to_ascii();
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(skin, deserialized);

    let skin = MadSkin::default_dark();
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(skin, deserialized);

    let skin = MadSkin::default_light();
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(skin, deserialized);

    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[2].set_fg(gray(22));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.quote_mark.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;
    skin.inline_code.add_attr(Attribute::Reverse);
    skin.paragraph.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.italic.add_attr(Attribute::Underlined);
    skin.italic.add_attr(Attribute::OverLined);
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(skin, deserialized);
}
