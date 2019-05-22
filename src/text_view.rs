use std::fmt;
use std::io;
use crossterm::{self, TerminalCursor, Terminal, ClearType};
use minimad::Line;

use crate::skin::MadSkin;
use crate::area::Area;
use crate::text::FormattedText;

pub struct TextView<'a, 't> {
    area: &'a Area,
    text: &'t FormattedText<'t, 't>,
    content_height: i32,
    scroll: i32, // 0 for no scroll, positive if scrolled
    pub show_scrollbar: bool,
}

impl<'a, 't> TextView<'a, 't> {
    /// make a displayed text, that is a text in an area
    pub fn from(
        area: &'a Area,
        text: &'t FormattedText,
    ) -> TextView<'a, 't> {
        TextView {
            area,
            text,
            content_height: text.text.lines.len() as i32,
            scroll: 0,
            show_scrollbar: true,
        }
    }
    // return an option which when filled contains
    //  a tupple with the top and bottom of the vertical
    //  scrollbar. Return none when the content fits
    //  the available space (or if show_scrollbar is false).
    pub fn scrollbar(&self) -> Option<(u16, u16)> {
        if !self.show_scrollbar {
            return None;
        }
        let h = self.area.height as i32;
        if self.content_height <= h {
            return None;
        }
        let sbh = h * h / self.content_height;
        let sc = self.scroll * h / self.content_height;
        Some((sc as u16, (sc + sbh + 1).min(i32::from(self.area.height+1)) as u16))
    }
    /// display the text in the area, taking the scroll into account.
    pub fn write(&self) -> io::Result<()> {
        let terminal = Terminal::new();
        let cursor = TerminalCursor::new();
        let scrollbar = self.scrollbar();
        let sx = self.area.left + self.area.width;
        let mut i = self.scroll as usize;
        for y in 0..=self.area.height {
            cursor.goto(self.area.left, self.area.top+y)?;
            terminal.clear(ClearType::UntilNewLine)?;
            if i < self.text.text.lines.len() {
                let fl = DisplayableLine::new(
                    &self.text.skin,
                    &self.text.text.lines[i],
                );
                print!("{}", fl);
                i += 1;
            }
            if let Some((sctop, scbottom)) = scrollbar {
                cursor.goto(sx, self.area.top+y)?;
                if sctop <= y && y <= scbottom {
                    println!("{}", self.text.skin.scrollbar.thumb);
                } else {
                    println!("{}", self.text.skin.scrollbar.track);
                }
            }
        }
        Ok(())
    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, lines_count: i32) {
        self.scroll = (self.scroll + lines_count)
            .max(0)
            .min(self.content_height - (self.area.height as i32) + 1);

    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_pages(&mut self, pages_count: i32) {
        self.try_scroll_lines(pages_count * self.area.height as i32);
    }
}

//------------------------
//

struct DisplayableLine<'s, 'l, 'p> {
    skin: &'s MadSkin,
    line: &'p Line<'l>,
}

impl<'s, 'l, 'p> DisplayableLine<'s, 'l, 'p> {
    pub fn new(skin: &'s MadSkin, line: &'p Line<'l>) -> DisplayableLine<'s, 'l, 'p> {
        DisplayableLine {
            skin,
            line
        }
    }
}

impl fmt::Display for DisplayableLine<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.skin.fmt_line(f, self.line)
    }
}
