use {
    crate::*,
};

/// The style to apply to an ordered list item, including the index and the suffix.
///
/// Usually depends on the depth/level
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrderedItemStyle {
    /// The style to apply to the index (number) of the ordered item
    pub index_style: CompoundStyle,
    /// The character that follows the index, usually a dot or a parenthesis
    pub index_suffix: StyledChar,
}


impl Default for OrderedItemStyle {
    fn default() -> Self {
        Self {
            index_style: CompoundStyle::default(),
            index_suffix: StyledChar::from_fg_char(gray(12), '.'),
        }
    }
}

impl OrderedItemStyle {
    pub fn no_style() -> Self {
        Self {
            index_style: CompoundStyle::default(),
            index_suffix: StyledChar::nude('.'),
        }
    }
    pub fn blend_with<C: Into<coolor::Color> + Copy>(&mut self, color: C, weight: f32) {
        self.index_style.blend_with(color, weight);
        self.index_suffix.blend_with(color, weight);
    }
}


pub fn ordered_item_indent(
    level: u8,
    index: u32,
) -> usize {
    index_width(index) + 2 + level as usize // +2: dot and space after the index
}

/// Number of decimal digits in `index` (at least 1, even for 0).
fn index_width(index: u32) -> usize {
    // This implementation doesn't use ilog10 which isn't available before Rust 1.67, and doesn't
    // use division or loops, which are slower. It also handles index == 0 correctly, returning 1.
    if index < 10 {
        1
    } else if index < 100 {
        2
    } else if index < 1_000 {
        3
    } else if index < 10_000 {
        4
    } else if index < 100_000 {
        5
    } else if index < 1_000_000 {
        6
    } else if index < 10_000_000 {
        7
    } else if index < 100_000_000 {
        8
    } else if index < 1_000_000_000 {
        9
    } else {
        10
    }
}

#[test]
fn test_index_width() {
    assert_eq!(index_width(0), 1);
    assert_eq!(index_width(1), 1);
    assert_eq!(index_width(9), 1);
    assert_eq!(index_width(10), 2);
    assert_eq!(index_width(99), 2);
    assert_eq!(index_width(100), 3);
}
