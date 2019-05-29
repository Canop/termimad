use std::io;
use crossterm::{self, TerminalCursor, Terminal, ClearType};

use crate::area::Area;
use crate::displayable_line::DisplayableLine;
use crate::text::FmtText;

/// a scrollable text, in a specific area.
pub struct TextView<'a, 't> {
    area: &'a Area,
    text: &'t FmtText<'t, 't>,
    pub scroll: i32, // 0 for no scroll, positive if scrolled
    pub show_scrollbar: bool,
}

impl<'a, 't> TextView<'a, 't> {
    /// make a displayed text, that is a text in an area
    pub fn from(
        area: &'a Area,
        text: &'t FmtText<'_, '_>,
    ) -> TextView<'a, 't> {
        TextView {
            area,
            text,
            scroll: 0,
            show_scrollbar: true,
        }
    }
    #[inline(always)]
    pub fn content_height(&self) -> i32 {
        self.text.lines.len() as i32
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
        if self.content_height() <= h {
            return None;
        }
        let sbh = h * h / self.content_height();
        let sc = self.scroll * h / self.content_height();
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
            if i < self.text.lines.len() {
                let dl = DisplayableLine::new(
                    self.text.skin,
                    &self.text.lines[i]
                );
                print!("{}", &dl);
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
        //for y in 0..=self.area.height {
        //    cursor.goto(self.area.left, self.area.top+y)?;
        //    terminal.clear(ClearType::UntilNewLine)?;
        //    if i < self.text.lines.len() {
        //        print!("{}", &self.text.lines[i]);
        //        i += 1;
        //    }
        //    if let Some((sctop, scbottom)) = scrollbar {
        //        cursor.goto(sx, self.area.top+y)?;
        //        if sctop <= y && y <= scbottom {
        //            println!("{}", self.text.skin.scrollbar.thumb);
        //        } else {
        //            println!("{}", self.text.skin.scrollbar.track);
        //        }
        //    }
        //}
        Ok(())
    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, lines_count: i32) {
        self.scroll = (self.scroll + lines_count)
            .max(0)
            .min(self.content_height() - (self.area.height as i32) + 1);

    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_pages(&mut self, pages_count: i32) {
        self.try_scroll_lines(pages_count * self.area.height as i32);
    }
}
