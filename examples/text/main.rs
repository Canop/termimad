use crossterm::{Color::*, Terminal, ClearType};
use termimad::*;

static MD: &str = r#"
# Markdown Rendering on Terminal

Here's the code to print this markdown block in the terminal:

    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    println!("{}", skin.term_text(my_markdown));

**Termimad** is built over **Crossterm** and **Minimad**.

## Why use termimad

* *display* static or dynamic *rich* texts
* *separate* your text building code or resources from its styling
* *configure* your colors
*
## Real use cases

* the help screen of a terminal application
* small app output or small in app snippets
"#;

fn print_direct(skin: &MadSkin) {
    println!("\n");
    println!("{}", skin.term_text(MD));
}

fn print_in_mad_view(skin: MadSkin) {
    let terminal = Terminal::new();
    terminal.clear(ClearType::All).unwrap();
    let mut area = Area::full_screen();
    area.pad(2, 1); // let's add some margin
    let view = MadView::from(MD.to_owned(), area, skin);
    view.write().unwrap();
}

/// Choose DIRECT = truefor a simple writting in stdout,
/// and DIRECT = false for a whole terminal display.
/// Note that this doesn't use an alternate screen. Look
/// at the "scrollable" example to see an alternate screen
/// being used.
const DIRECT: bool = true;

fn main() {
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    if DIRECT {
        print_direct(&skin);
    } else {
        print_in_mad_view(skin);
    }
}
