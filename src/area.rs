
use crossterm::Terminal;

pub trait AreaContent {
    fn height() -> u16;
}

/// represents a part of a screen
#[derive(Debug, PartialEq, Eq)]
pub struct Area {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
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
    // return the last line of the area (included)
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
}
