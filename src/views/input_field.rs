use {
    crate::{Area, CompoundStyle, Error, Event},
    std::io::Write,
    crossterm::{
        cursor,
        event::{
            KeyCode,
            KeyEvent,
        },
        queue,
        style::Attribute,
    },
};

/// A simple input field, managing its cursor position.
pub struct InputField {
    content: Vec<char>,
    pub area: Area,
    cursor_pos: usize, // position in chars
    normal_style: CompoundStyle,
    cursor_style: CompoundStyle,
}

impl InputField {
    pub fn new(area: Area) -> Self {
        debug_assert!(area.height == 1, "input area must be of height 1");
        let normal_style = CompoundStyle::default();
        let mut cursor_style = normal_style.clone();
        cursor_style.add_attr(Attribute::Reverse);
        Self {
            content: Vec::new(),
            area,
            cursor_pos: 0,
            normal_style,
            cursor_style,
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
    /// apply the passed event to change the state (content, cursor)
    ///
    /// Return true when the event was used.
    pub fn apply_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Click(x, y, ..) if *y == self.area.top + 1 && *x > self.area.left => {
                let p = (x - 1 - self.area.left) as usize;
                self.cursor_pos = p.min(self.content.len());
                true
            }
            Event::Key(KeyEvent{code, modifiers}) if modifiers.is_empty() => {
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
                        self.put_char(*c);
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
        queue!(w, cursor::MoveTo(self.area.left, self.area.top))?;
        for (i, c) in self.content.iter().enumerate() {
            if self.cursor_pos == i {
                self.cursor_style.queue(w, c)?;
            } else {
                self.normal_style.queue(w, c)?;
            }
        }
        let mut e = self.content.len();
        if e == self.cursor_pos {
            self.cursor_style.queue(w, ' ')?;
            e += 1;
        }
        for _ in e..self.area.width as usize {
            self.normal_style.queue(w, ' ')?;
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
