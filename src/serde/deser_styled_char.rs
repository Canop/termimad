use {
    crate::{
        StyledChar,
        parse_styled_char,
    },
    serde::de,
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


