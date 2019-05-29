use crossterm::{Color::*, Terminal};
use termimad::*;

static MD_TABLE: &str = r#"
|:-:|:-:|-
|**feature**|**supported**|**details**|
|-:|:-:|-
| tables | yes | pipe based only, with alignments
| italic, bold | yes | star based |
| inline code | yes | `with backquotes` (it works in tables too)
| code bloc | yes |with tabs: Fences not supported
| horizontal rule |  not yet
| crossed text |  not yet | ---this isn't crossed---
| lists | yes|* unordered lists supported
|  | |* ordered lists *not* supported
| quotes |  not yet
| phpbb like links | no | (because it's preferable to show an URL in a terminal)
|-
"#;

fn main() {
    println!("\n");
    let mut skin = MadSkin::new();
    skin.set_headers_fg_color(Rgb{r:255, g:187, b:0});
    mad_fg!(skin.bold, Yellow);
    mad_colors!(skin.italic, Magenta, Rgb{r:30, g:30, b:40});
    let (width, _) = Terminal::new().terminal_size();
    let mut markdown = format!(" Available width: *{}*", width);
    markdown.push_str(MD_TABLE);
    println!("{}", skin.term_text(&markdown));
    println!("\n");
}

