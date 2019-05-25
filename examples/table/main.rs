extern crate termimad;

use crossterm::{Color::*};
use termimad::*;

static MD_INTRO: &str = r#"
# Wrapped Table
"#;
static MD_TABLE: &str = r#"
|feature|supported|details|
| tables | yes | pipe based only, alignement not yet supported
| italic, bold | yes | star based only|
| inline code | yes |
| code bloc | yes |with tabs. Fences not supported
| crossed text |  not yet
| phpbb like links | no | (because it's preferable to show an URL in a terminal)
"#;

fn main() {
    println!("\n");
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    let text = skin.terminal_wrapped_text(MD_TABLE);
    println!("{}", text);
    println!("\n");
}

