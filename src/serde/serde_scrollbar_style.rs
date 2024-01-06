use {
    crate::{
        CompoundStyle,
        ScrollBarStyle,
        StyledChar,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};

/// A variable-complexity definition of a scrollbar,
/// allowing a simplified representation covering most
/// cases.
///
/// You should not use this enum unless you're writing
/// your own skin type Serialize/Deserialize impls.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScrollBarStyleDef {
    Simple(StyledChar),
    Rich {
        track: StyledChar,
        thumb: StyledChar,
    },
}

impl From<&ScrollBarStyle> for ScrollBarStyleDef {
    fn from(sc: &ScrollBarStyle) -> Self {
        let simple = sc.track.nude_char() == sc.thumb.nude_char()
            && sc.track.get_bg().is_none()
            && sc.thumb.get_bg().is_none();
        if simple {
            Self::Simple(StyledChar::new(
                CompoundStyle::new(
                    sc.thumb.get_fg(),
                    sc.track.get_fg(),
                    None,
                    Default::default(),
                ),
                sc.track.nude_char(),
            ))
        } else {
            Self::Rich {
                track: sc.track.clone(),
                thumb: sc.thumb.clone(),
            }
        }
    }
}

impl ScrollBarStyleDef {
    pub fn into_scrollbar_style(self) -> ScrollBarStyle {
        match self {
            Self::Simple(sc) => sc.into(),
            Self::Rich{ track, thumb } => ScrollBarStyle { track, thumb },
        }
    }
}
