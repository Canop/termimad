use crossterm::{ObjectStyle, Attribute, Color};

use crate::formatted::FormattedLine;

pub struct MadSkin {
    pub normal: ObjectStyle,
    pub bold: ObjectStyle,
    pub italic: ObjectStyle,
    pub code: ObjectStyle,
}


impl MadSkin {
    pub fn new() -> MadSkin {
        let mut skin = MadSkin {
            normal: ObjectStyle::new(),
            bold: ObjectStyle::new(),
            italic: ObjectStyle::new(),
            code: ObjectStyle::new(),
        };
        skin.bold.add_attr(Attribute::Bold);
        skin.italic.add_attr(Attribute::Italic);
        skin.code = skin.code.bg(Color::Rgb{r:40, g:40, b:40});
        skin
    }
    pub fn line<'s, 'l>(&'s self, src: &'l str) -> FormattedLine<'s, 'l> {
        FormattedLine::new(self, src)
    }
    pub fn println(&self, src: &str) {
        println!("{}", FormattedLine::new(self, src));
    }
}
