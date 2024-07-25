

/// Describe how list items are indented when the item has to be
/// wrapped: either only the first line (the one with the bullet)
/// or the whole item as a block.
///
/// FirstLineOnly is more compact, but Block is prettier and more
/// readable.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ListItemsIndentationMode {
    FirstLineOnly,
    #[default]
    Block,
}
