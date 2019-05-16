

extern crate termimad;

use crossterm::{Attribute, Color, ObjectStyle};
use termimad::MadSkin;

fn show(skin: &MadSkin, src: &str){
    println!(" Raw       : {}", &src);
    println!(" Formatted : {}\n", skin.line(src));
}

fn show_some(skin: &MadSkin){
    show(&skin, "*Hey* **World!** Here's `some(code)`");
    show(&skin, "some *nested **style***");
}

fn main(){
    println!();
    println!("\nWith the default skin:\n");
    let mut skin = MadSkin::new();
    show_some(&skin);
    println!("\nWith a customized skin:\n");
    skin.bold = skin.bold.fg(Color::Yellow);
    skin.italic = ObjectStyle::new().bg(Color::DarkBlue);
    skin.code.add_attr(Attribute::Reverse);
    show_some(&skin);
}
