/// The set of characters to use to render table borders
#[derive(Debug, Clone, PartialEq)]
pub struct TableBorderChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left_corner: char,
    pub top_right_corner: char,
    pub bottom_right_corner: char,
    pub bottom_left_corner: char,
    pub top_junction: char,
    pub right_junction: char,
    pub bottom_junction: char,
    pub left_junction: char,
    pub cross: char,
}

impl TableBorderChars {
    /// return a key which could be used in by_key.
    ///
    /// (note that we're comparing the values, not the pointers)
    pub fn key(&self) -> Option<&'static str> {
        if self == STANDARD_TABLE_BORDER_CHARS {
            Some("standard")
        } else if self == ASCII_TABLE_BORDER_CHARS {
            Some("ascii")
        } else if self == ROUNDED_TABLE_BORDER_CHARS {
            Some("rounded")
        } else {
            None
        }
    }
    pub fn by_key(key: &str) -> Option<&'static Self> {
        match key {
            "standard" => Some(STANDARD_TABLE_BORDER_CHARS),
            "ascii" => Some(ASCII_TABLE_BORDER_CHARS),
            "rounded" => Some(ROUNDED_TABLE_BORDER_CHARS),
            _ => None,
        }
    }
}

/// Default square tables
pub static STANDARD_TABLE_BORDER_CHARS: &TableBorderChars = &TableBorderChars {
    horizontal: '─',
    vertical: '│',
    top_left_corner: '┌',
    top_right_corner: '┐',
    bottom_right_corner: '┘',
    bottom_left_corner: '└',
    top_junction: '┬',
    right_junction: '┤',
    bottom_junction: '┴',
    left_junction: '├',
    cross: '┼',
};

/// For tables made only of ASCII (not extended)
///
/// It's automatically used when you call `skin.limit_to_ascii()`
pub static ASCII_TABLE_BORDER_CHARS: &TableBorderChars = &TableBorderChars {
    horizontal: '-',
    vertical: '|',
    top_left_corner: '+',
    top_right_corner: '+',
    bottom_right_corner: '+',
    bottom_left_corner: '+',
    top_junction: '+',
    right_junction: '+',
    bottom_junction: '+',
    left_junction: '+',
    cross: '+',
};

/// Allow tables to be more rounded
///
/// ```
/// let mut skin = termimad::MadSkin::default();
/// skin.table_border_chars = termimad::ROUNDED_TABLE_BORDER_CHARS;
/// ```
pub static ROUNDED_TABLE_BORDER_CHARS: &TableBorderChars = &TableBorderChars {
    horizontal: '─',
    vertical: '│',
    top_left_corner: '╭',
    top_right_corner: '╮',
    bottom_right_corner: '╯',
    bottom_left_corner: '╰',
    top_junction: '┬',
    right_junction: '┤',
    bottom_junction: '┴',
    left_junction: '├',
    cross: '┼',
};
