use {
    crate::{
        Area,
        CompoundStyle,
        Error,
        Event,
        fit,
    },
    std::io::Write,
    crossterm::{
        cursor,
        event::{
            KeyCode,
            KeyEvent,
            KeyModifiers,
        },
        queue,
        style::{
            Attribute,
            Color,
            SetBackgroundColor,
        },
    },
};

/// A simple input field, managing its cursor position.
pub struct InputField {
    content: Vec<char>,
    pub area: Area,
    cursor_pos: usize, // position in chars
    normal_style: CompoundStyle,
    cursor_style: CompoundStyle,
    pub focused: bool,
}

impl InputField {
    pub fn new(area: Area) -> Self {
        debug_assert!(area.height == 1, "input area must be of height 1");
        let normal_style = CompoundStyle::default();
        let mut cursor_style = normal_style.clone();
        cursor_style.add_attr(Attribute::Reverse);
        let focused = true;
        Self {
            content: Vec::new(),
            area,
            cursor_pos: 0,
            normal_style,
            cursor_style,
            focused,
        }
    }
    pub fn change_area(&mut self, x: u16, y: u16, w: u16) {
        self.area.left = x;
        self.area.top = y;
        self.area.width = w;
    }
    pub fn set_normal_style(&mut self, style: CompoundStyle) {
        self.normal_style = style;
        self.cursor_style = self.normal_style.clone();
        self.cursor_style.add_attr(Attribute::Reverse);
    }
    pub fn get_content(&self) -> String {
        self.content.iter().collect()
    }
    /// tell whether the content of the input is equal
    ///  to the argument
    pub fn is_content(&self, s: &str) -> bool {
        // TODO this comparison could be optimized
        let str_content = self.get_content();
        str_content == s
    }
    /// change the content to the new one and
    ///  put the cursor at the end **if** the
    ///  content is different from the previous one.
    pub fn set_content(&mut self, s: &str) {
        if self.is_content(s) {
            return;
        }
        self.content = s.chars().collect();
        self.cursor_pos = self.content.len();
    }
    /// put a char at cursor position (and increments this
    /// position)
    pub fn put_char(&mut self, c: char) {
        self.content.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }
    /// remove the char left of the cursor, if any
    pub fn del_char_left(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.content.remove(self.cursor_pos);
            true
        } else {
            false
        }
    }
    /// remove the char at cursor position, if any
    pub fn del_char_below(&mut self) -> bool {
        if self.cursor_pos < self.content.len() {
            self.content.remove(self.cursor_pos);
            true
        } else {
            false
        }
    }

    /// apply a click event
    ///
    /// (for when you handle the events yourselves and don't
    ///  have a termimad event)
    pub fn apply_click_event(&mut self, x: u16, y: u16) -> bool {
        if self.area.contains(x, y) {
            if self.focused {
                let p = (x - 1 - self.area.left) as usize;
                self.cursor_pos = p.min(self.content.len());
            } else {
                self.focused = true;
            }
            true
        } else {
            false
        }
    }

    /// apply an event being a key without modifier.
    ///
    /// You don't usually call this function but the more
    /// general `apply_event`. This one is useful when you
    /// manage events yourselves.
    pub fn apply_keycode_event(&mut self, code: KeyCode) -> bool {
        if !self.focused {
            return false;
        }
        match code {
            KeyCode::Home => {
                self.cursor_pos = 0;
                true
            }
            KeyCode::End => {
                self.cursor_pos = self.content.len();
                true
            }
            KeyCode::Char(c) => {
                self.put_char(c);
                true
            }
            KeyCode::Left if self.cursor_pos > 0 => {
                self.cursor_pos -= 1;
                true
            }
            KeyCode::Right if self.cursor_pos < self.content.len() => {
                self.cursor_pos += 1;
                true
            }
            KeyCode::Backspace => self.del_char_left(),
            KeyCode::Delete => self.del_char_below(),
            _ => false,
        }

    }

    /// apply the passed event to change the state (content, cursor)
    ///
    /// Return true when the event was used.
    pub fn apply_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Click(x, y, ..) => {
                self.apply_click_event(*x, *y)
            }
            Event::Key(KeyEvent{code, modifiers}) if (modifiers.is_empty()||*modifiers==KeyModifiers::SHIFT) => {
                self.apply_keycode_event(*code)
            }
            _ => false,
        }
    }

    /// render the input field on screen.
    ///
    /// All rendering must be explicitely called, no rendering is
    /// done on functions changing the state.
    ///
    /// w is typically either stderr or stdout.
    pub fn display_on<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        queue!(w, SetBackgroundColor(Color::Reset))?;
        queue!(w, cursor::MoveTo(self.area.left, self.area.top))?;

        let mut slice_start = 0;
        let width = self.area.width as usize;
        let mut ellipsis_at_start = false;
        let mut ellipsis_at_end = false;
        if self.content.len() + 1 >= width {
            if self.cursor_pos <= width / 2 {
                slice_start = 0;
                ellipsis_at_end = true;
            } else if self.cursor_pos >= self.content.len() - width / 2 {
                slice_start = self.content.len() + 1 - width;
                ellipsis_at_start = true;
            } else {
                slice_start = self.cursor_pos - width / 2;
                ellipsis_at_start = true;
                ellipsis_at_end = true;
            }
        }
        for i in 0..width {
            if i == 0 && ellipsis_at_start {
                self.normal_style.queue(w, fit::ELLIPSIS)?;
                continue;
            }
            if i == width-1 && ellipsis_at_end {
                self.normal_style.queue(w, fit::ELLIPSIS)?;
                continue;
            }
            let idx = i + slice_start;
            if idx >= self.content.len() {
                if self.focused && (idx==self.cursor_pos) && (idx==self.content.len()) {
                    self.cursor_style.queue(w, ' ')?;
                } else {
                    self.normal_style.queue(w, ' ')?;
                }
            } else {
                let c = self.content[idx];
                if self.focused && (self.cursor_pos == idx) {
                    self.cursor_style.queue(w, c)?;
                } else {
                    self.normal_style.queue(w, c)?;
                }
            }
        }
        Ok(())
    }

    /// render the input field on stdout
    pub fn display(&self) -> Result<(), Error> {
        let mut w = std::io::stdout();
        self.display_on(&mut w)?;
        w.flush()?;
        Ok(())
    }
}
