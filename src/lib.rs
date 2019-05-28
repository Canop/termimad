/// This crate lets you display simple markdown snippets in the terminal.
///
#[macro_use]
extern crate lazy_static;

mod composite;
mod area;
mod code;
mod line;
mod inline;
mod skin;
mod tbl;
mod text;
mod text_view;
mod wrap;
mod mad_view;
mod displayable_line;

pub use area::Area;
pub use skin::MadSkin;
pub use composite::FmtComposite;
pub use line::FmtLine;
pub use inline::FmtInline;
pub use text::FmtText;
pub use text_view::TextView;
pub use mad_view::MadView;

fn get_default_skin<'s>() -> &'s MadSkin {
    lazy_static! {
        static ref DEFAULT_SKIN: MadSkin = MadSkin::new();
    }
    &DEFAULT_SKIN
}

/// return a formatted line, which implements Display
/// This uses the default skin.
/// Don't use if you expect your markdown to be several lines.
pub fn inline(src: &str) -> FmtInline {
    get_default_skin().inline(src)
}

/// return a formatted text, which implements Display
/// This uses the default skin and doesn't wrap the lines
///  at all. Most often you'll prefer to use `term_text`
///  which makes a text wrapped for the current terminal.
pub fn text(src: &str) -> FmtText {
    get_default_skin().text(src, None)
}

/// return a formatted text, which implements Display
/// This uses the default skin and the terminal's width
pub fn term_text(src: &str) -> FmtText {
    get_default_skin().term_text(src)
}

pub fn print_inline(src: &str) {
    get_default_skin().print_inline(src);
}

pub fn print_text(src: &str) {
    get_default_skin().print_text(src);
}
