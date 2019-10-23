use crossterm::Terminal;
use std::process;

/// A default width which is used when we failed measuring the real terminal width
const DEFAULT_TERMINAL_WIDTH: u16 = 50;

/// A default height which is used when we failed measuring the real terminal width
const DEFAULT_TERMINAL_HEIGHT: u16 = 20;

pub trait AreaContent {
    fn height() -> u16;
}

/// A rectangular part of the screen
#[derive(Debug, PartialEq, Eq)]
pub struct Area {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

fn div_ceil(a: i32, b: i32) -> i32 {
    a / b + if a % b != 0 { 1 } else { 0 }
}

impl Area {
    /// Build a new area. You'll need to set the position and size
    /// before you can use it
    pub fn uninitialized() -> Area {
        Area {
            left: 0,
            top: 0,
            height: 1,
            width: 5,
        } // width can't be less than 5
    }

    /// build a new area.
    pub fn new(left: u16, top: u16, width: u16, height: u16) -> Area {
        assert!(width > 4);
        Area {
            left,
            top,
            width,
            height,
        }
    }

    /// Build an area covering the whole terminal
    pub fn full_screen() -> Area {
        let (width, height) = terminal_size();
        Area {
            left: 0,
            top: 0,
            width,
            height,
        }
    }

    /// shrink the area
    pub fn pad(&mut self, dx: u16, dy: u16) {
        // this will crash if padding is too big. feature?
        self.left += dx;
        self.top += dy;
        self.width -= 2 * dx;
        self.height -= 2 * dy;
    }

    /// symmetrically shrink the area if its width is bigger than `max_width`
    pub fn pad_for_max_width(&mut self, max_width: u16) {
        if max_width >= self.width {
            return;
        }
        let pw = self.width - max_width;
        self.left += pw / 2;
        self.width -= pw;
    }

    /// Return an option which when filled contains
    ///  a tupple with the top and bottom of the vertical
    ///  scrollbar. Return none when the content fits
    ///  the available space.
    pub fn scrollbar(
        &self,
        scroll: i32, // 0 for no scroll, positive if scrolled
        content_height: i32,
    ) -> Option<(u16, u16)> {
        compute_scrollbar(scroll, content_height, self.height as i32)
    }
}

pub fn compute_scrollbar(
    scroll: i32,           // 0 for no scroll, positive if scrolled
    content_height: i32,   // number of lines of the content
    available_height: i32, // for an area it's usually its height
) -> Option<(u16, u16)> {
    let h = available_height;
    if content_height <= h {
        return None;
    }
    let sc = div_ceil(scroll * h, content_height);
    let hidden_tail = content_height - scroll - h;
    let se = div_ceil(hidden_tail * h, content_height);
    Some((
        sc as u16,
        if h > sc + se {
            (h - se) as u16
        } else {
            sc as u16 + 1
        },
    ))
}

/// execute tput with the given argument and parse
/// the output as a u16.
///
/// The arg should be "cols" or "lines"
fn tput_value(arg: &str) -> Option<u16> {
    match process::Command::new("tput").arg(arg).output() {
        Ok(process::Output { stdout, .. }) => {
            let value = stdout
                .iter()
                .map(|&b| b as u16)
                .take_while(|&b| b >= 48 && b <= 58)
                .fold(0, |v, b| v * 10 + (b - 48));
            if value > 0 {
                Some(value)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Return a (width, height) with the dimensions of the available
/// terminal in characters.
///
pub fn terminal_size() -> (u16, u16) {
    let size = Terminal::new().size();
    #[cfg(unix)]
    {
        if size.is_err() {
            if let (Some(w), Some(h)) = (tput_value("cols"), tput_value("lines")) {
                return (w, h);
            }
        }
    }
    size.unwrap_or((DEFAULT_TERMINAL_WIDTH, DEFAULT_TERMINAL_HEIGHT))
}
