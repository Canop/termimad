use std::io;

use crossterm::Terminal;

pub trait AreaContent {
    fn height() -> u16;
}

/// a part of a screen
#[derive(Debug, PartialEq, Eq)]
pub struct Area {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {

    /// build a new area. You'll need to set the position and size
    /// before you can use it
    pub fn uninitialized() -> Area {
        Area { left: 0, top:0, height:1, width:5 } // width can't be less than 5
    }

    /// build a new area.
    pub fn new(
        left: u16,
        top: u16,
        width: u16,
        height: u16,
    ) -> Area {
        assert!(width > 4);
        Area {
            left,
            top,
            width,
            height,
        }
    }

    /// build an area covering the whole terminal
    pub fn full_screen() -> Area {
        let (width, height) = Terminal::new().terminal_size();
        Area {
            left: 0,
            top: 0,
            width,
            height,
        }
    }

    /// return the last line of the area (included)
    pub fn bottom(&self) -> u16 {
        return self.top + self.height - 1;
    }

    pub fn pad(&mut self, dx: u16, dy: u16) {
        // this will crash if padding is too big. feature?
        self.left += dx;
        self.top += dy;
        self.width -= 2*dx;
        self.height -= 2*dy;
    }

    // return an option which when filled contains
    //  a tupple with the top and bottom of the vertical
    //  scrollbar. Return none when the content fits
    //  the available space.
    pub fn scrollbar(
        &self,
        scroll: i32, // 0 for no scroll, positive if scrolled
        content_height: i32,
    ) -> Option<(u16, u16)> {
        let h = self.height as i32;
        if content_height <= h {
            return None;
        }
        let sbh = h * h / content_height;
        let sc = scroll * h / content_height;
        Some((sc as u16, (sc + sbh + 1).min(h+1) as u16))
    }
}
