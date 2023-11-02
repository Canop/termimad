//! this example prints a skin as JSON to stdout.
//!
//! Run it with `cargo run --example serialize-skin`
use {
    termimad::{
        ansi,
        gray,
        ROUNDED_TABLE_BORDER_CHARS,
        rgb,
        StyledChar,
        minimad::Alignment,
    },
    crossterm::style::{
        Attribute,
        Color::*,
    },
};

/// Set this to true if you want to see a skin with a lot of changes
/// from the default, set it to false to export the default skin
const FANCY: bool = true;

fn main() {
    let mut skin = termimad::MadSkin::default();
    if FANCY {
        skin.set_headers_fg(AnsiValue(178));
        skin.headers[0].set_bg(ansi(129));
        skin.headers[1].add_attr(Attribute::Bold);
        // note: gray(22) will appear as ansi(254) in the output,
        // it's the same color
        skin.headers[2].set_fg(gray(22));
        skin.bold.set_fg(Yellow);
        skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
        skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
        skin.quote_mark.set_fg(Yellow);
        skin.italic.set_fg(Magenta);
        skin.scrollbar.thumb.set_fg(AnsiValue(178));
        skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
        skin.paragraph.align = Alignment::Center;
        skin.table.align = Alignment::Center;
        skin.inline_code.add_attr(Attribute::Reverse);
        skin.paragraph.set_fg(Magenta);
        skin.italic.add_attr(Attribute::Underlined);
        skin.italic.add_attr(Attribute::OverLined);
    }
    let serialized = serde_json::to_string_pretty(&skin).unwrap();
    println!("{serialized}");
}
