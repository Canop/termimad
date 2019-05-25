extern crate termimad;

use crossterm::{Attribute::*, Color::*, ObjectStyle};
use termimad::*;

fn show(skin: &MadSkin, src: &str) {
    println!(" Raw       : {}", &src);
    println!(" Formatted : {}\n", skin.inline(src));
}

fn show_some(skin: &MadSkin) {
    show(&skin, "*Hey* **World!** Here's `some(code)`");
    show(&skin, "some *nested **style***");
}

fn main() {
    println!();
    println!("\nWith the default skin:\n");
    let mut skin = MadSkin::new();
    show_some(&skin);
    println!("\nWith a customized skin:\n");
    mad_fg!(skin.bold, Yellow);
    skin.italic = ObjectStyle::new().bg(DarkBlue);
    skin.code.add_attr(Reverse);
    show_some(&skin);

    let mut skin = MadSkin::new();
    skin.bold = skin.bold.fg(Yellow);
    skin.print_inline("*Hey* **World!** Here's `some(code)`");
    mad_colors!(skin.paragraph, Magenta, Rgb{r:30, g:30, b:40});
    skin.italic.add_attr(Underlined);
    skin.italic.add_attr(OverLined);
    println!("\nand now {}\n", skin.inline("a little *too much* **style!** (and `some(code)` too)"));
}
