/// This crate lets you display simple markdown snippets in the terminal.
///
#[macro_use]
extern crate lazy_static;

mod area;
mod code;
mod displayable_line;
mod line;
mod skin;
mod tbl;
mod text;
mod text_view;
mod wrap;

pub use area::Area;
pub use skin::MadSkin;
pub use line::FormattedLine;
pub use text::FormattedText;
pub use text_view::TextView;

fn get_default_skin<'s>() -> &'s MadSkin {
    lazy_static! {
        static ref DEFAULT_SKIN: MadSkin = MadSkin::new();
    }
    &DEFAULT_SKIN
}

/// return a formatted line, which implements Display
/// This uses the default skin.
/// Don't use if you expect your markdown to be several lines.
pub fn inline(src: &str) -> FormattedLine {
    get_default_skin().inline(src)
}

/// return a formatted text, which implements Display
/// This uses the default skin.
pub fn text(src: &str) -> FormattedText {
    get_default_skin().text(src)
}


pub fn print_inline(src: &str) {
    get_default_skin().print_inline(src);
}

pub fn print_text(src: &str) {
    get_default_skin().print_text(src);
}
