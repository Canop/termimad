use {
    crate::{
        LineStyle,
        parse_line_style,
        parse::PushStyleTokens,
    },
    serde::{de, Serialize, Serializer},
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

impl Serialize for LineStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_style_tokens_string())
    }
}
