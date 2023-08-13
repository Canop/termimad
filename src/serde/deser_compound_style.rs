use {
    crate::{
        CompoundStyle,
        parse_compound_style,
    },
    serde::de,
};

impl<'de> de::Deserialize<'de> for CompoundStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_compound_style(&s).map_err(de::Error::custom)
    }
}

