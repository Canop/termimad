#[macro_use]
extern crate lazy_static;

mod skin;
mod formatted;

pub use skin::MadSkin;

pub fn println(src: &str){
        lazy_static! {
            static ref DEFAULT_SKIN: MadSkin = MadSkin::new();
        }
        DEFAULT_SKIN.println(src);
}

