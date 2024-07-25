use {
    crate::*,
    minimad::*,
};

/// The global kind of a composite
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompositeKind {
    Paragraph,
    Header(u8),
    ListItem(u8),
    ListItemFollowUp(u8),
    Code,
    Quote,
}


impl From<CompositeStyle> for CompositeKind {
    fn from(ty: CompositeStyle) -> Self {
        match ty {
            CompositeStyle::Paragraph => Self::Paragraph,
            CompositeStyle::Header(level) => Self::Header(level),
            CompositeStyle::ListItem(level) => Self::ListItem(level),
            CompositeStyle::Code => Self::Code,
            CompositeStyle::Quote => Self::Quote,
        }
    }
}
