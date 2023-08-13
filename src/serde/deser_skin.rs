use {
    crate::{
        minimad::Alignment,
        ATTRIBUTES,
        LineStyle,
        MadSkin,
        parse_compound_style,
        parse_styled_char,
        parse_line_style,
    },
    serde::{de, Deserialize},
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
                            // this simple deserialization covers the most frequent
                            // cases but don't offer the whole freedom of the
                            // ScrollbarStyle struct. I'll enable a more detailled
                            // syntax if requested
                            let value = map.next_value::<String>()?;
                            let sc = parse_styled_char(&value, '▐')
                                .map_err(de::Error::custom)?;
                            skin.scrollbar = sc.into();
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

#[derive(Deserialize)]
#[serde(untagged)]
enum HeadersStyleInfo {
    Add(LineStyle),
    Levels(Vec<LineStyle>),
}
