extern crate termimad;

use crossterm::{Color::*};
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

fn main() {
    println!("\n");
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    println!("{}", skin.term_text(MD));
    println!("\n");
}
