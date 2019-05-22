extern crate termimad;

use crossterm::{style, AlternateScreen, ObjectStyle, Terminal, TerminalCursor, TerminalInput, KeyEvent, InputEvent, ClearType, Color::*};
use termimad::*;
use std::{io, thread, time};

fn show_scrollable(skin: &MadSkin, text: &str) -> io::Result<()> {
    let terminal = Terminal::new();
    let cursor = TerminalCursor::new();
    cursor.hide()?;
    let mut area = Area::full_screen();
    area.left += 2; // let's add some margin
    area.top += 2;
    area.width -= 4;
    area.height -= 4;
    let text = skin.wrapped_text(text, &area);
    let mut text_view = TextView::from(&area, &text);
    text_view.show_scrollbar = true;
    let mut crossterm_events = TerminalInput::new().read_sync();
    loop {
        terminal.clear(ClearType::All)?;
        text_view.write();
        if let Some(InputEvent::Keyboard(key)) = crossterm_events.next() {
            match key {
                KeyEvent::Up => {
                    text_view.try_scroll(-1);
                }
                KeyEvent::Down => {
                    text_view.try_scroll(1);
                }
                _ => {
                    break;
                }
            }
        }
    }
    cursor.show()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    skin.scrollbar.set_track_fg(Rgb{r:30, g:30, b:40});
    skin.scrollbar.set_thumb_fg(Rgb{r:67, g:51, b:0});
    skin
}

fn main() {
    let alt_screen = AlternateScreen::to_alternate(true);
    let skin = make_skin();
    show_scrollable(&skin, MD).unwrap();
}

static MD: &str = r#"# Scrollable Area Example

Use the **🡑** and **🡓** arrow keys to scroll this page.
Use any other key to quit the application.

*Now I'll describe this example with more words than necessary, in order to be sure to demonstrate scrolling (and **wrapping**, too, thanks to long sentences).

## What's shown

* an **area** fitting the screen (with some side margin to be prettier)
* a markdown text **parsed**, **skinned**, **wrapped** to fit the width
* a **scrollable** view
* the whole in *raw terminal mode*

## Usage

* **🡑** and **🡓** arrow keys : scroll this page
* any other key : quit

## Demonstrated features

Raw terminal Support
*
Here's the code to print this markdown block in the terminal:
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    println!("{}", skin.text(my_markdown));
**Termimad** is built over **Crossterm** and **Minimad**.
## Why use termimad
* *display* static or dynamic *rich* text
* *separate* your text building code or resources from its styling
* *configure* your colors
## Real use cases
* the help screen of a terminal application
* small app output or small in app snippets
## Not (yet) supported:
* tables
* non star based styles
* fenced code blocs
* and many other things, to be honest
## Why use termimad
* *display* static or dynamic *rich* text
* *separate* your text building code or resources from its styling
* *configure* your colors
## Real use cases
* the help screen of a terminal application
* small app output or small in app snippets
## Not (yet) supported:
* tables
* non star based styles
* fenced code blocs
* and many other things, to be honest
"#;

