
/// The set of characters to use to render table borders
#[derive(Debug, Clone)]
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

