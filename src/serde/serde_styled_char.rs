use {
    crate::{
        parse::PushStyleTokens,
        parse_styled_char,
        StyledChar,
    },
    serde::{
        de,
        Serialize,
        Serializer,
    },
};

impl<'de> de::Deserialize<'de> for StyledChar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_styled_char(&s, '*').map_err(de::Error::custom)
    }
}

impl Serialize for StyledChar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_style_tokens_string())
    }
}
