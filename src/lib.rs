/*! This crate lets you display simple markdown snippets
or scrollable wrapped markdown texts in the terminal.

In order to use Termimad you typically need
* some *markdown*, a string which you can have loaded or dynamically built
* a *skin*, which defines the colors and style attributes of every parts

Additionnaly, you might define an *area* of the screen in which to draw (and maybe scroll).

# The skin

It's an instance of [`MadSkin`](struct.MadSkin.html) whose fields you customize according
to your tastes or (better) to your application's configuration.


```rust
use crossterm::{Color::*, Attribute::*};
use termimad::*;

// start with the default skin
let mut skin = MadSkin::default();
// let's decide bold is in light gray
skin.bold.set_fg(gray(20));
// let's make strikeout not striked out but red
skin.strikeout = CompoundStyle::new(Some(Red), None, vec![Bold]);
```

**Beware:**
* you may define colors in full [`rgb`](fn.rgb.html) but this will limit compatibility with old
terminals. It's recommended to stick to [Ansi colors](fn.ansi.html), [gray levels](fn.gray.html), or [Crossterm predefined values](https://docs.rs/crossterm/0.9.6/crossterm/enum.Color.html).
* styles are composed. For example a word may very well be italic, bold and striked out. It might not be wise to have them differ only by their background color for example.

# Display a simple inline snippet


```
use crossterm::TerminalCursor;
# use termimad::*;
# let skin = MadSkin::default();

// with the default skin, nothing simpler:
termimad::print_inline("value: **52**");

// now in a precise position
// (assuming an alternate terminal, see the "scrollable" example)
// and with the skin we precedently customized:

let cursor = TerminalCursor::new();
cursor.goto(0, 4).unwrap();
skin.print_inline("page *3* / 5");

```
# Print a text

A multi-line markdown string can be printed the same way than an *inline* snippet, but you usually want it to be wrapped according to the available terminal width.

```rust,no_run
# use termimad::*;
# let skin = MadSkin::default();
# let my_markdown = "#title\n* item 1\n* item 2";
println!("{}", skin.term_text(my_markdown));
```

`MadSkin` contains other functions to prepare a text for no specific size or for one which isn't the terminal's width.

# Display a text, maybe scroll it

A terminal application often uses an *alternate* screen instead of just dumping its text to stdout, and you often want to display in a specific rect of that screen, with adequate wrapping and not writing outside that rect.

You may also want to display a scrollbar if the text doesn't fit the area. A [`MadView`](struct.MadView.html) makes that simple:

```
# use termimad::*;
# let markdown = String::from("#title\n* item 1\n* item 2");
# let skin = MadSkin::default();
let area = Area::new(0, 0, 10, 12);
let mut view = MadView::from(markdown, area, skin);
view.write().unwrap();
```

If you don't want give ownership of the skin, markdown and area, you may prefer to use a [`TextView`](struct.TextView.html).

You may see how to write a text viewer responding to key inputs to scroll a markdown text in [the scrollable example](https://github.com/Canop/termimad/blob/master/examples/scrollable/main.rs).

The repository contains several other examples, which hopefully cover the whole API while being simple enough. It's recommended you start by trying them or at least glance at their code.

*/


#[macro_use]
extern crate lazy_static;

mod area;
mod code;
mod color;
mod composite;
mod compound_style;
mod displayable_line;
mod events;
mod inline;
mod line;
mod line_style;
mod scrollbar_style;
mod styled_char;
mod skin;
mod spacing;
mod tbl;
mod text;
mod views;
mod wrap;

pub use area::{Area, compute_scrollbar, terminal_size};
pub use color::{gray, ansi, rgb};
pub use composite::FmtComposite;
pub use compound_style::CompoundStyle;
pub use events::{Event, EventSource};
pub use inline::FmtInline;
pub use line::FmtLine;
pub use line_style::LineStyle;
pub use minimad::Alignment;
pub use scrollbar_style::ScrollBarStyle;
pub use skin::MadSkin;
pub use styled_char::StyledChar;
pub use spacing::Spacing;
pub use text::FmtText;
pub use views::{InputField, ListView, ListViewCell, ListViewColumn, MadView, TextView};

/// Return a reference to the global skin (modifiable).
///
/// If you want a new default skin without messing with
/// the other default printings, get a separate instance
/// with `Skin::default()` instead.
pub fn get_default_skin() -> &'static MadSkin {
    lazy_static! {
        static ref DEFAULT_SKIN: MadSkin = MadSkin::default();
    }
    &DEFAULT_SKIN
}

/// Return a formatted line, which implements Display.
///
/// This uses the default skin.
/// Don't use if you expect your markdown to be several lines.
pub fn inline(src: &str) -> FmtInline<'_, '_> {
    get_default_skin().inline(src)
}

/// Return an unwrapped formatted text, implementing Display.
///
/// This uses the default skin and doesn't wrap the lines
///  at all. Most often you'll prefer to use `term_text`
///  which makes a text wrapped for the current terminal.
pub fn text(src: &str) -> FmtText<'_, '_> {
    get_default_skin().text(src, None)
}

/// Return a terminal wrapped formatted text, implementing Display.
///
/// This uses the default skin and the terminal's width
pub fn term_text(src: &str) -> FmtText<'_, '_> {
    get_default_skin().term_text(src)
}

/// Write a string interpreted as markdown with the default skin.
pub fn print_inline(src: &str) {
    get_default_skin().print_inline(src);
}

/// Write a text interpreted as markdown with the default skin.
pub fn print_text(src: &str) {
    get_default_skin().print_text(src);
}
