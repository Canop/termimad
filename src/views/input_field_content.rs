use {
    std::{
        fmt,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Line {
    pub chars: Vec<char>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputFieldContent {
    /// the cursor's position
    pos: Pos,
    /// never empty
    lines: Vec<Line>,
}

pub struct Chars<'c> {
    content: &'c InputFieldContent,
    pos: Pos,
}
impl Iterator for Chars<'_> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let line = &self.content.lines[self.pos.y];
        if self.pos.x < line.chars.len() {
            self.pos.x += 1;
            Some(line.chars[self.pos.x - 1])
        } else if self.pos.y + 1 < self.content.lines.len() {
            self.pos.y += 1;
            self.pos.x = 0;
            Some('\n')
        } else {
            None
        }
    }
}
impl<'c> IntoIterator for &'c InputFieldContent {
    type Item = char;
    type IntoIter = Chars<'c>;
    fn into_iter(self) -> Self::IntoIter {
        Chars {
            content: self,
            pos: Pos::default(),
        }
    }
}


impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;
        for &c in &self.chars {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Default for InputFieldContent {
    fn default() -> Self {
        Self {
            // there's always a line
            lines: vec![Line::default()],
            pos: Pos::default(),
        }
    }
}

impl fmt::Display for InputFieldContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;
        let mut lines = self.lines.iter().peekable();
        loop {
            if let Some(line) = lines.next() {
                for &c in &line.chars {
                    f.write_char(c)?;
                }
                if lines.peek().is_some() {
                    f.write_char('\n')?;
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl InputFieldContent {
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    pub fn line(&self, y: usize) -> Option<&Line> {
        self.lines.get(y)
    }
    pub fn current_line(&self) -> &Line {
        &self.lines[self.pos.y]
    }
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }
    pub fn cursor_pos(&self) -> Pos {
        self.pos
    }
    /// Set the cursor position.
    ///
    /// The position set may be different to ensure consistency
    /// (for example if it's after the end, it will be set back).
    pub fn set_cursor_pos(&mut self, new_pos: Pos) {
        if new_pos.y >= self.lines.len() {
            self.pos = self.end();
        } else {
            self.pos.y = new_pos.y;
            self.pos.x = new_pos.x.min(self.lines[self.pos.y].chars.len());
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.lines.len() {
            1 => self.lines[0].chars.is_empty(),
            _ => false,
        }
    }
    /// return the position on end, where the cursor should be put
    /// initially
    pub fn end(&self) -> Pos {
        let y = self.lines.len() - 1;
        Pos { x:self.lines[y].chars.len(), y }
    }
    fn last_line(&mut self) -> &mut Line {
        let y = self.lines.len() - 1;
        &mut self.lines[y]
    }
    /// add a char at end, without updating the position.
    ///
    /// This shouldn't be used in normal event handling as
    /// characters are normally inserted on insertion point
    /// with insert_char.
    pub fn push_char(&mut self, c: char) {
        if c == '\n' {
            self.lines.push(Line::default());
        } else {
            self.last_line().chars.push(c);
        }
    }
    /// Initialize from a string, with the cursor at end
    pub fn from<S: AsRef<str>>(s: S) -> Self {
        let mut content = Self::default();
        content.insert_str(s);
        content
    }
    pub fn clear(&mut self) {
        self.lines.clear();
        self.lines.push(Line::default());
        self.pos = Pos::default();
    }
    pub fn insert_new_line(&mut self) {
        let new_line = Line {
            chars: self.lines[self.pos.y].chars.split_off(self.pos.x),
        };
        self.pos.x = 0;
        self.pos.y += 1;
        self.lines.insert(self.pos.y, new_line);
    }
    /// Insert a character at the current position, updating
    /// this position
    pub fn insert_char(&mut self, c: char) {
        if c == '\n' {
            self.insert_new_line();
        } else {
            self.lines[self.pos.y].chars.insert(self.pos.x, c);
            self.pos.x += 1;
        }
    }
    /// Insert the string on cursor point, as if it was typed
    pub fn insert_str<S: AsRef<str>>(&mut self, s: S) {
        for c in s.as_ref().chars() {
            self.insert_char(c);
        }
    }
    /// Tell whether the content of the input is equal to the argument,
    /// comparing char by char
    pub fn is_str(&self, s: &str) -> bool {
        let mut ia = self.into_iter();
        let mut ib = s.chars();
        loop {
            match (ia.next(), ib.next()) {
                (Some(a), Some(b)) if a == b => { continue }
                (None, None) => { return true; }
                _ => { return false; }
            }
        }
    }
    /// change the content to the new one and put the cursor at the end **if** the
    ///  content is different from the previous one.
    ///
    ///  Don't move the cursor if the string content didn't change.
    pub fn set_str<S: AsRef<str>>(&mut self, s: S) {
        if self.is_str(s.as_ref()) {
            return;
        }
        self.clear();
        self.insert_str(s);
    }
    /// remove the char left of the cursor, if any.
    pub fn del_char_left(&mut self) -> bool {
        if self.pos.x > 0 {
            self.pos.x -= 1;
            self.lines[self.pos.y].chars.remove(self.pos.x);
            true
        } else if self.pos.y > 0 {
            let mut removed_line = self.lines.remove(self.pos.y);
            self.pos.y -= 1;
            self.pos.x = self.lines[self.pos.y].chars.len();
            self.lines[self.pos.y].chars.append(&mut removed_line.chars);
            true
        } else {
            false
        }
    }
    /// Remove the char at cursor position, if any.
    ///
    /// Cursor position is unchanged
    pub fn del_char_below(&mut self) -> bool {
        let line_len = self.current_line().chars.len();
        if line_len == 0 {
            if self.lines.len() > 1 {
                self.lines.remove(self.pos.y);
                true
            } else {
                false
            }
        } else if self.pos.x < line_len {
            self.lines[self.pos.y].chars.remove(self.pos.x);
            true
        } else {
            false
        }
    }
    /// Move the cursor to the right (or to the line below
    /// if it's a the end of a non-last line)
    pub fn move_right(&mut self) -> bool {
        if self.pos.x < self.lines[self.pos.y].chars.len() {
            self.pos.x += 1;
            true
        } else {
            false
        }
    }
    pub fn move_lines_up(&mut self, lines: usize) -> bool {
        if self.pos.y > 0 {
            self.pos.y -= lines.min(self.pos.y);
            let line_len = self.lines[self.pos.y].chars.len();
            if self.pos.x > line_len {
                self.pos.x = line_len;
            }
            true
        } else {
            false
        }
    }
    pub fn move_up(&mut self) -> bool {
        self.move_lines_up(1)
    }
    pub fn move_lines_down(&mut self, lines: usize) -> bool {
        if self.pos.y + 1 < self.lines.len() {
            self.pos.y += lines.min(self.lines.len() - self.pos.y - 1);
            let line_len = self.lines[self.pos.y].chars.len();
            if self.pos.x > line_len {
                self.pos.x = line_len;
            }
            true
        } else {
            false
        }
    }
    pub fn move_down(&mut self) -> bool {
        self.move_lines_down(1)
    }
    pub fn move_left(&mut self) -> bool {
        if self.pos.x > 0 {
            self.pos.x -= 1;
            true
        } else {
            false
        }
    }
    pub fn move_to_end(&mut self) -> bool {
        let pos = self.end();
        if pos == self.pos {
            false
        } else {
            self.pos = pos;
            true
        }
    }
    pub fn move_to_start(&mut self) -> bool {
        let pos = Pos { x: 0, y: 0 };
        if pos == self.pos {
            false
        } else {
            self.pos = pos;
            true
        }
    }
    pub fn move_to_line_end(&mut self) -> bool {
        let line_len = self.lines[self.pos.y].chars.len();
        if self.pos.x < line_len {
            self.pos.x = line_len;
            true
        } else {
            false
        }
    }
    pub fn move_to_line_start(&mut self) -> bool {
        if self.pos.x > 0 {
            self.pos.x = 0;
            true
        } else {
            false
        }
    }
    pub fn move_word_left(&mut self) -> bool {
        if self.pos.x > 0 {
            let chars = &self.lines[self.pos.y].chars;
            loop {
                self.pos.x -= 1;
                if self.pos.x == 0 || !chars[self.pos.x-1].is_alphanumeric() {
                    break;
                }
            }
            true
        } else {
            false
        }
    }
    pub fn move_word_right(&mut self) -> bool {
        if self.pos.x < self.lines[self.pos.y].chars.len() {
            let chars = &self.lines[self.pos.y].chars;
            loop {
                self.pos.x += 1;
                if self.pos.x +1 >= chars.len() || !chars[self.pos.x+1].is_alphanumeric() {
                    break;
                }
            }
            true
        } else {
            false
        }
    }
    pub fn del_word_left(&mut self) -> bool {
        if self.pos.x > 0 {
            let chars = &mut self.lines[self.pos.y].chars;
            loop {
                self.pos.x -= 1;
                chars.remove(self.pos.x);
                if self.pos.x == 0 || !chars[self.pos.x-1].is_alphanumeric() {
                    break;
                }
            }
            true
        } else {
            false
        }
    }
    /// Delete the word rigth of the cursor.
    ///
    // I'm not yet sure of what should be the right behavior but all changes
    // should be discussed from cases defined as in the unit tests below
    pub fn del_word_right(&mut self) -> bool {
        let chars = &mut self.lines[self.pos.y].chars;
        if self.pos.x < chars.len() {
            loop {
                let deleted_is_an = chars[self.pos.x].is_alphanumeric();
                chars.remove(self.pos.x);
                if !deleted_is_an {
                    break;
                }
                if self.pos.x == chars.len() {
                    if self.pos.x > 0 {
                        self.pos.x -= 1;
                    }
                    break;
                }
            }
            true
        } else if self.pos.x == self.current_line().chars.len() && self.pos.x > 0 {
            self.pos.x -= 1;
            true
        } else {
            false
        }
    }

}

#[test]
fn test_char_iterator() {
    let texts = vec![
        "this has\nthree lines\n",
        "",
        "123",
        "\n\n",
    ];
    for text in texts {
        assert!(InputFieldContent::from(text).is_str(text));
    }
}

#[cfg(test)]
mod input_content_edit_monoline_tests {

    use super::*;

    /// make an input for tests from two strings:
    /// - the content string (no wide chars)
    /// - a cursor position specified as a string with a caret
    fn make_content(value: &str, cursor_pos: &str) -> InputFieldContent {
        let mut content = InputFieldContent::from(value);
        content.pos = Pos {
            x: cursor_pos.chars().position(|c| c=='^').unwrap(),
            y: 0,
        };
        content
    }

    fn check(a: &InputFieldContent, value: &str, cursor_pos: &str) {
        let b = make_content(value, cursor_pos);
        assert_eq!(a, &b);
    }

    /// test the behavior of new line insertion
    #[test]
    fn test_new_line() {
        let mut con = make_content(
            "12345",
            "  ^  "
        );
        con.insert_char('6');
        check(
            &con,
            "126345",
            "   ^  ",
        );
        con.insert_new_line();
        assert!(con.is_str("126\n345"));
        let mut con = InputFieldContent::default();
        con.insert_char('1');
        con.insert_char('2');
        con.insert_new_line();
        con.insert_char('3');
        con.insert_char('4');
        assert!(con.is_str("12\n34"));
    }

    /// test the behavior of del_word_right
    #[test]
    fn test_del_word_right() {
        let mut con = make_content(
            "aaa bbb ccc",
            "     ^     ",
        );
        con.del_word_right();
        check(
            &con,
            "aaa bccc",
            "     ^  ",
        );
        con.del_word_right();
        check(
            &con,
            "aaa b",
            "    ^",
        );
        con.del_word_right();
        check(
            &con,
            "aaa ",
            "   ^",
        );
        con.del_word_right();
        check(
            &con,
            "aaa",
            "   ^",
        );
        con.del_word_right();
        check(
            &con,
            "aaa",
            "  ^",
        );
        con.del_word_right();
        check(
            &con,
            "aa",
            " ^",
        );
        con.del_word_right();
        check(
            &con,
            "a",
            "^",
        );
        con.del_word_right();
        check(
            &con,
            "",
            "^",
        );
        con.del_word_right();
        check(
            &con,
            "",
            "^",
        );
    }
}

