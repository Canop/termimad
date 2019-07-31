use std::io;
use crossterm_cursor::TerminalCursor;
use crossterm_terminal::{Terminal, ClearType};

use crate::area::Area;
use crate::displayable_line::DisplayableLine;
use crate::text::FmtText;

/// A scrollable text, in a specific area.
///
/// The text is assumed to have been computed for the given area.
///
/// For example:
///
/// ```
/// use termimad::*;
///
/// // You typically borrow those 3 vars from elsewhere
/// let markdown = "#title\n* item 1\n* item 2";
/// let area = Area::new(0, 0, 10, 12);
/// let skin = MadSkin::default();
///
/// // displaying
/// let text = skin.area_text(markdown, &area);
/// let view = TextView::from(&area, &text);
/// view.write().unwrap();
/// ```
///
/// If the text and skin are constant, you might prefer to
/// use a MadView instead of a TextView: the MadView owns
/// the mardkown string and ensures the formatted text
/// is computed accordingly to the area.
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

    /// return an option which when filled contains
    ///  a tupple with the top and bottom of the vertical
    ///  scrollbar. Return none when the content fits
    ///  the available space (or if show_scrollbar is false).
    #[inline(always)]
    pub fn scrollbar(&self) -> Option<(u16, u16)> {
        if self.show_scrollbar {
            self.area.scrollbar(self.scroll, self.content_height())
        } else {
            None
        }
    }

    /// display the text in the area, taking the scroll into account.
    pub fn write(&self) -> io::Result<()> {
        let terminal = Terminal::new();
        let cursor = TerminalCursor::new();
        let scrollbar = self.scrollbar();
        let sx = self.area.left + self.area.width;
        let mut i = self.scroll as usize;
        for y in 0..self.area.height {
            cursor.goto(self.area.left, self.area.top+y)?;
            if i < self.text.lines.len() {
                let dl = DisplayableLine::new(
                    self.text.skin,
                    &self.text.lines[i],
                    self.text.width,
                );
                print!("{}", &dl);
                i += 1;
            } else {
                terminal.clear(ClearType::UntilNewLine)?;
            }
            if let Some((sctop, scbottom)) = scrollbar {
                cursor.goto(sx, self.area.top+y)?;
                if sctop <= y && y <= scbottom {
                    print!("{}", self.text.skin.scrollbar.thumb);
                } else {
                    print!("{}", self.text.skin.scrollbar.track);
                }
            }
        }
        Ok(())
    }

    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, lines_count: i32) {
        self.scroll = (self.scroll + lines_count)
            .min(self.content_height() - (self.area.height as i32) + 1)
            .max(0);
    }
    /// set the scroll amount.
    /// pages_count can be negative
    pub fn try_scroll_pages(&mut self, pages_count: i32) {
        self.try_scroll_lines(pages_count * self.area.height as i32);
    }
}
