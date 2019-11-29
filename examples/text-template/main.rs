/*!
This example demonstrates the use of templates for building
whole texts.

You execute this example with
     cargo run --example text-template
*/
use std::io::Write;

use crossterm::style::{Attribute::*, Color::*};
use minimad::TextTemplate;

#[macro_use]
use termimad::*;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate minimad;

static TEMPLATE: &str = r#"
-----------
# ${app-name} v${app-version}
**${app-name}** is *fantastic*!
## Modules
In a table:
|:-:|:-:|:-:|
|**name**|**path**|**description**|
|-:|:-:|:-|
${module-rows
|**${module-name}**|`${app-version}/${module-key}`|${module-description}|
}
|-|-|-|
and the same data in another form:
${module-rows
### ${module-name} (${module-key})**
${module-description}
}
## Items
${formatted-items
* **${item-name}:** `${item-code}`
}
## Example of a code block
    ${some-function}
-----------
"#;

fn main() -> Result<()> {
    let skin = make_skin();

    let text_template = TextTemplate::from(TEMPLATE);
    let mut expander = text_template.expander();
    expander
        .set("app-name", "MyApp")
        .set("app-version", "42.5.3");
    expander.sub("module-rows")
        .set("module-name", "lazy-regex")
        .set("module-key", "lrex")
        .set("module-description", "eases regexes");
    expander.sub("module-rows")
        .set("module-name", "termimad")
        .set("module-key", "tmd")
        .set_md("module-description", "do things on *terminal*");
    expander.sub("formatted-items")
        .set("item-name", "3*5")
        .set("item-code", "187/12");
    expander.sub("formatted-items")
        .set("item-name", "π")
        .set("item-code", "22/7");
    expander.set_lines("some-function", r#"
        fun test(a rational) {
            irate(a)
        }
        "#);
    let text = expander.expand();
    let (width, _) = terminal_size();
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    println!("{}", &fmt_text);
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[2].set_fg(gray(22));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}
