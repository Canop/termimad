use std::io;
use crossterm::{self, TerminalCursor, Terminal, ClearType};

use crate::area::Area;
use crate::text::FmtText;
use crate::text_view::TextView;
use crate::skin::MadSkin;

/// A MadView is like a textview but it owns everything, from the
///  source markdown to the area and the skin, which often makes it more convenient
///  for dynamic texts.
/// It's also resizeable.
pub struct MadView {
    markdown: String,
    area: Area,
    pub skin: MadSkin,
    scroll: i32,
}

impl MadView {
    //pub fn new(src: String) -> MadView {
    //}
    /// make a displayed text, that is a text in an area
    pub fn from(
        markdown: String,
        area: Area,
        skin: MadSkin,
    ) -> MadView {
        MadView {
            markdown,
            area,
            skin,
            scroll: 0,
        }
    }
    pub fn write(&self) -> io::Result<()> {
        let text = self.skin.text(&self.markdown, Some(self.area.width as usize));
        //let text = self.skin.area_wrapped_text(&self.markdown, &self.area);
        let mut text_view = TextView::from(&self.area, &text);
        text_view.scroll = self.scroll;
        text_view.write()
    }
    /// sets the new area. If it's the same as the precedent one,
    ///  this operation does nothing. The scroll is kept if possible.
    pub fn resize(&mut self, area: &Area) {
        if *area == self.area {
            return;
        }
        if area.width != self.area.width {
            self.scroll = 0; //TODO improve
        }
        self.area.left = area.left;
        self.area.top = area.top;
        self.area.height = area.height;
        self.area.width = area.width;
    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, lines_count: i32) {
        let text = self.skin.text(&self.markdown, Some(self.area.width as usize));
        let mut text_view = TextView::from(&self.area, &text);
        text_view.scroll = self.scroll;
        text_view.try_scroll_lines(lines_count);
        self.scroll = text_view.scroll;
    }
    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_pages(&mut self, pages_count: i32) {
        self.try_scroll_lines(pages_count * self.area.height as i32);
    }
}

