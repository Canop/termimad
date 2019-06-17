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
| crossed text |  ~~not yet~~ | wait... now it works (`~~like this~~`)
| lists | yes|* unordered lists supported
|  | |* ordered lists *not* supported
| quotes |  not yet
| phpbb like links | no | (because it's preferable to show an URL in a terminal)
|-
"#;

fn main() {
    println!("\n");
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb!(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb!(30, 30, 40));
    let (width, _) = terminal_size();
    let mut markdown = format!(" Available width: *{}*", width);
    markdown.push_str(MD_TABLE);
    println!("{}", skin.term_text(&markdown));
    println!("\n");
}

