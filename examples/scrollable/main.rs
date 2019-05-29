use crossterm::{AlternateScreen, TerminalCursor, TerminalInput, KeyEvent::*, InputEvent::*, Color::*};
use termimad::*;
use std::io;

fn show_scrollable(skin: MadSkin, markdown: &str) -> io::Result<()> {
    let cursor = TerminalCursor::new();
    cursor.hide()?;
    let mut area = Area::full_screen();
    area.pad(2, 1); // let's add some margin
    let mut view = MadView::from(markdown.to_owned(), area, skin);
    let mut events = TerminalInput::new().read_sync();
    loop {
        view.write()?;
        if let Some(Keyboard(key)) = events.next() {
            match key {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
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
    let _alt_screen = AlternateScreen::to_alternate(true);
    let skin = make_skin();
    show_scrollable(skin, MD).unwrap();
}

static MD: &str = r#"# Scrollable Markdown in Termimad

Use the **🡑** and **🡓** arrow keys to scroll this page.
Use any other key to quit the application.

*Now I'll describe this example with more words than necessary, in order to be sure to demonstrate scrolling (and **wrapping**, too, thanks to long sentences).

## What's shown

* an **area** fitting the screen (with some side margin to be prettier)
* a markdown text **parsed**, **skinned**, **wrapped** to fit the width
* a **scrollable** view in *raw terminal mode*

## Area

The area specifies the part of the screen where we'll display our markdown. The margin in this example is just here to show that wrapping is handled:

    let mut area = Area::full_screen();
    area.pad(2, 1); // let's add some margin

## Parsed Markdown

The text is parsed from a string. In this example we directly wrap it for the width of the area:

    let text = skin.area_wrapped_text(markdown, &area);

If we wanted to modify the parsed representation, or modify the area width, we could also have kept the parsed text (*but parsing is cheap*).

## The TextView

It's just a text put in an area, tracking your **scroll** position (and whether you want the scrollbar to be displayed).

    let mut text_view = TextView::from(&area, &text);

## Really Scrolling

Not two applications handle events in the same way. **Termimad** doesn't try to handle this but lets you write it yourself, which is fairly easily done with **Crossterm** for example:

    let mut events = TerminalInput::new().read_sync();
    loop {
        text_view.write()?;
        if let Some(Keyboard(key)) = events.next() {
            match key {
                Up => text_view.try_scroll_lines(-1),
                Down => text_view.try_scroll_lines(1),
                PageUp => text_view.try_scroll_pages(-1),
                PageDown => text_view.try_scroll_pages(1),
                _ => break,
            }
        }
    }

## Skin

We want *shiny **colors***:

    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    mad_fg!(skin.bold, Yellow);
    skin.scrollbar.set_track_fg(Rgb{r:30, g:30, b:40});
    skin.scrollbar.set_thumb_fg(Rgb{r:67, g:51, b:0});

The scrollbar's colors were also adjusted to be consistent.

## Usage

* **🡑** and **🡓** arrow keys : scroll this page
* any other key : quit

## And let's just finish by a table

It's a little out of context but it shows how a wide table can be wrapped in a thin terminal.

|feature|supported|details|
| tables | yes | pipe based only, alignement not yet supported
| italic, bold | yes | star based only|
| inline code | yes |
| code bloc | yes |with tabs. Fences not supported
| crossed text |  not yet
| phpbb like links | no | (because it's preferable to show an URL in a terminal)
"#;

