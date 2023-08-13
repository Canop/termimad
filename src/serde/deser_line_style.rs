use {
    crate::{
        LineStyle,
        parse_line_style,
    },
    serde::de,
};

impl<'de> de::Deserialize<'de> for LineStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_line_style(&s).map_err(de::Error::custom)
    }
}
