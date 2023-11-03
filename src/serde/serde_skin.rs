use {
    super::ScrollBarStyleDef,
    crate::{
        minimad::Alignment,
        ATTRIBUTES,
        LineStyle,
        MadSkin,
        parse_compound_style,
        parse_styled_char,
        parse_line_style,
        TableBorderChars,
    },
    serde::{
        de,
        Deserialize,
        Serialize,
        Serializer,
        ser::SerializeMap,
    },
    std::fmt,
};

impl<'de> de::Deserialize<'de> for MadSkin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct SkinVisitor;

        impl<'de> de::Visitor<'de> for SkinVisitor {
            type Value = MadSkin;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("MadSkin")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut skin = MadSkin::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {

                        // inline styles
                        "bold" => {
                            let value = map.next_value::<String>()?;
                            let cs = parse_compound_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.bold = cs;
                        }
                        "italic" => {
                            let value = map.next_value::<String>()?;
                            let cs = parse_compound_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.italic = cs;
                        }
                        "strikeout" => {
                            let value = map.next_value::<String>()?;
                            let cs = parse_compound_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.strikeout = cs;
                        }
                        "inline_code" | "inline-code" => {
                            let value = map.next_value::<String>()?;
                            let cs = parse_compound_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.inline_code = cs;
                        }
                        "ellipsis" => {
                            let value = map.next_value::<String>()?;
                            let cs = parse_compound_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.ellipsis = cs;
                        }

                        // marker chars
                        "bullet" => {
                            let value = map.next_value::<String>()?;
                            let sc = parse_styled_char(&value, '*')
                                .map_err(de::Error::custom)?;
                            skin.bullet = sc;
                        }
                        "quote_mark" | "quote" | "quote-mark" => {
                            let value = map.next_value::<String>()?;
                            let sc = parse_styled_char(&value, '*')
                                .map_err(de::Error::custom)?;
                            skin.quote_mark = sc;
                        }
                        "horizontal_rule" | "horizontal-rule" | "rule" => {
                            let value = map.next_value::<String>()?;
                            let sc = parse_styled_char(&value, '*')
                                .map_err(de::Error::custom)?;
                            skin.horizontal_rule = sc;
                        }

                        // scrollbar
                        "scrollbar" => {
                            let def: ScrollBarStyleDef = map.next_value()?;
                            skin.scrollbar = def.into_scrollbar_style();
                        }

                        // line styles
                        "paragraph" => {
                            let value = map.next_value::<String>()?;
                            let ls = parse_line_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.paragraph = ls;
                        }
                        "code_block" | "code-block" => {
                            let value = map.next_value::<String>()?;
                            let ls = parse_line_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.code_block = ls;
                        }
                        "table" => {
                            let value = map.next_value::<String>()?;
                            let ls = parse_line_style(&value)
                                .map_err(de::Error::custom)?;
                            skin.table = ls;
                        }

                        // headers
                        "headers" => {
                            match map.next_value::<HeadersStyleInfo>()? {
                                HeadersStyleInfo::Add(ls) => {
                                    for h in &mut skin.headers {
                                        if let Some(fg) = ls.compound_style.get_fg() {
                                            h.compound_style.set_fg(fg);
                                        }
                                        if let Some(bg) = ls.compound_style.get_bg() {
                                            h.compound_style.set_bg(bg);
                                        }
                                        for &attr in ATTRIBUTES {
                                            if ls.compound_style.has_attr(attr) {
                                                h.compound_style.add_attr(attr);
                                            }
                                        }
                                        if ls.align != Alignment::Unspecified {
                                            h.align = ls.align;
                                        }
                                    }
                                }
                                HeadersStyleInfo::Levels(mut vls) => {
                                    for (lvl, h) in vls.drain(..).enumerate() {
                                        if lvl < skin.headers.len() {
                                            skin.headers[lvl] = h;
                                        }
                                    }
                                }
                            }
                        }

                        // table border chars
                        // There's currently no way to allow custom table border
                        // chars. It would require a change in MadSkin: either
                        // make it require a lifetime, or use a Cow for the border
                        // chars
                        "table_border_chars" | "table-border-chars" => {
                            let key = map.next_value::<String>()?;
                            if let Some(chars) = TableBorderChars::by_key(&key) {
                                skin.table_border_chars = chars;
                            }
                        }

                        _ => {
                            let _ = map.next_value::<String>()?;
                            println!("unknown key: {key}");
                        }
                    }
                }
                Ok(skin)
            }
        }

        deserializer.deserialize_map(SkinVisitor {})
    }
}

impl Serialize for MadSkin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut skin = serializer.serialize_map(None)?;

        // inline styles
        skin.serialize_entry("bold", &self.bold)?;
        skin.serialize_entry("italic", &self.italic)?;
        skin.serialize_entry("strikeout", &self.strikeout)?;
        skin.serialize_entry("inline_code", &self.inline_code)?;
        skin.serialize_entry("ellipsis", &self.ellipsis)?;

        // marker chars
        skin.serialize_entry("bullet", &self.bullet)?;
        skin.serialize_entry("quote", &self.quote_mark)?;
        skin.serialize_entry("horizontal_rule", &self.horizontal_rule)?;

        // scrollbar
        let def: ScrollBarStyleDef = (&self.scrollbar).into();
        skin.serialize_entry("scrollbar", &def)?;

        // line styles
        skin.serialize_entry("paragraph", &self.paragraph)?;
        skin.serialize_entry("code_block", &self.code_block)?;
        skin.serialize_entry("table", &self.table)?;

        // headers
        skin.serialize_entry("headers", &self.headers)?;

        // table border chars
        // There's currently no way to allow custom
        if let Some(key) = self.table_border_chars.key() {
            skin.serialize_entry("table_border_chars", key)?;
        }

        skin.end()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum HeadersStyleInfo {
    Add(LineStyle),
    Levels(Vec<LineStyle>),
}

/// Check that serializing a skin in JSON, then deserializing this
/// JSON into a new skin, results in an identical skin.
#[test]
fn skin_json_roundtrip() {
    use {
        crate::{
            gray,
            ROUNDED_TABLE_BORDER_CHARS,
            rgb,
            StyledChar,
        },
        crossterm::style::{
            Attribute,
            Color::*,
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
