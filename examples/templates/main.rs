use std::io::Write;
use crossterm::style::{Attribute::*, Color::*};

#[macro_use]
use termimad::*;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate minimad;

/// cargo run --example templates
fn main() -> Result<()> {
    let mut skin = MadSkin::default();
    skin.paragraph.set_bg(ansi(17));
    skin.bold.set_fg(Yellow);
    skin.inline_code.add_attr(Reverse);
    skin.italic.set_fg(White);
    let mut w = std::io::stdout();

    println!();

    // no interpolation, just markdown
    mad_print_inline!(&skin, "This is *Markdown*!");
    println!();

    // with interpolation
    mad_print_inline!(&skin, "*count:* **$0**", "27"); // note that only &str is supported now
    println!();

    // another one: see that the arguments aren't interpreted as markdown,
    //  which is convenient for user supplied texts
    mad_print_inline!(&skin, "**$0:** ` area = $2 ` and ` perimeter = $1 `", "Disk", "2*π*r", "π*r²");
    println!();

    // using any Write:
    mad_write_inline!(&mut w, &skin, "**$0** is *$1*", "Meow", "crazy").unwrap();
    println!();

    // looping: the template is compiled only once (the macro stores the compiled
    // template in a lazy static var)
    let user_supplied_strings = vec![
        "Victor Hugo",
        "L'escargot et l'alouette",
        "Pizza weight: π * z * z * a", // the stars don't mess with the markdown
    ];
    for (idx, string) in user_supplied_strings.iter().enumerate() {
        let idx = idx.to_string(); // the macro only accepts &str arguments
        mad_print_inline!(
            &skin,
            "Exhibit $0 : *$1*",
            &idx,
            string,
        );
        println!();
    }

    // usage of the minimad `mad_inline!` macro to get a composite allowing other operations
    let composite = mad_inline!(
        "**command:** `$0`",
        "cp -r /some/long/path/to/a/file /some/other/path",
    );
    // print in a longer space and align right
    skin.write_composite_fill(& mut w, composite.clone(), 70, Alignment::Right).unwrap();
    println!();
    // print in a short span -> smart ellision occurs
    skin.write_composite_fill(& mut w, composite.clone(), 40, Alignment::Left).unwrap();
    println!();
    w.flush()?;

    println!();
    println!();
    Ok(())
}
