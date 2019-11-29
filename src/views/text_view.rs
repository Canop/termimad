use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{Clear, ClearType},
};

use crate::area::Area;
use crate::displayable_line::DisplayableLine;
use crate::errors::Result;
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
    pub fn from(area: &'a Area, text: &'t FmtText<'_, '_>) -> TextView<'a, 't> {
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
    pub fn write(&self) -> Result<()> {
        let mut stdout = stdout();
        self.write_on(&mut stdout)?;
        stdout.flush()?;
        Ok(())
    }

    /// display the text in the area, taking the scroll into account.
    pub fn write_on<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        let scrollbar = self.scrollbar();
        let sx = self.area.left + self.area.width;
        let mut i = self.scroll as usize;
        for y in 0..self.area.height {
            queue!(w, MoveTo(self.area.left, self.area.top + y))?;
            if i < self.text.lines.len() {
                let dl = DisplayableLine::new(self.text.skin, &self.text.lines[i], self.text.width);
                write!(w, "{}", &dl)?;
                i += 1;
            }
            self.text.skin.paragraph.compound_style.queue_bg(w)?;
            queue!(w, Clear(ClearType::UntilNewLine))?;
            if let Some((sctop, scbottom)) = scrollbar {
                queue!(w, MoveTo(sx, self.area.top + y))?;
                if sctop <= y && y <= scbottom {
                    write!(w, "{}", self.text.skin.scrollbar.thumb)?;
                } else {
                    write!(w, "{}", self.text.skin.scrollbar.track)?;
                }
            }
        }
        Ok(())
    }

    /// set the scroll position but makes it fit into allowed positions.
    /// Return the actual scroll.
    pub fn set_scroll(&mut self, scroll: i32) -> i32 {
        self.scroll = scroll
            .min(self.content_height() - i32::from(self.area.height) + 1)
            .max(0);
        self.scroll
    }

    /// change the scroll position
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, lines_count: i32) -> i32{
        self.set_scroll(self.scroll + lines_count)
    }

    /// change the scroll position
    /// pages_count can be negative
    pub fn try_scroll_pages(&mut self, pages_count: i32) -> i32{
        self.try_scroll_lines(pages_count * i32::from(self.area.height))
    }
}
