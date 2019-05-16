use std::fmt::{self, Write};

use crossterm::ObjectStyle;
use minimad::Line;

use crate::skin::MadSkin;

pub struct FormattedLine<'s,'l> {
    skin: &'s MadSkin,
    line: Line<'l>,
}

impl<'s, 'l> FormattedLine<'s, 'l> {
    pub fn new(skin: &'s MadSkin, text: &'l str) -> FormattedLine<'s, 'l> {
        FormattedLine {
            skin,
            line: Line::from(text)
        }
    }
}


impl fmt::Display for FormattedLine<'_,'_> {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        for c in &self.line.compounds {
            // the following code is horrible. I don't really know how to
            // efficiently deal with multiple types of Display
            match (c.bold, c.italic, c.code) {
                (true, false, false) => {
                    let styled = self.skin.normal.apply_to(
                        self.skin.bold.apply_to(c.as_str())
                    );
                    write!(f, "{}", styled);
                }
                (false, true, false) => {
                    let styled = self.skin.normal.apply_to(
                        self.skin.italic.apply_to(c.as_str())
                    );
                    write!(f, "{}", styled);
                }
                (true, true, false) => {
                    let styled = self.skin.normal.apply_to(
                        self.skin.italic.apply_to(
                            self.skin.bold.apply_to(
                                c.as_str()
                            )
                        )
                    );
                    write!(f, "{}", styled);
                }
                (false, false, false) => {
                    let styled = self.skin.normal.apply_to(c.as_str());
                    write!(f, "{}", styled);
                }
                _ => {
                    let styled = self.skin.code.apply_to(c.as_str());
                    write!(f, "{}", styled);
                }
            }
        }
        Ok(())
    }
}
