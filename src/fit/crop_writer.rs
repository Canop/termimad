use {
    crate::{
        crossterm::{
            style::Print,
            QueueableCommand,
        },
        *,
    },
    std::borrow::Cow,
    unicode_width::UnicodeWidthChar,
};

/// wrap a writer to ensure that at most `allowed` columns are
/// written.
pub struct CropWriter<'w, W>
where
    W: std::io::Write,
{
    pub w: &'w mut W,

    /// number of screen columns which may be covered
    pub allowed: usize,

    /// the string replacing a tabulation
    pub tab_replacement: &'static str,
}

impl<'w, W> CropWriter<'w, W>
where
    W: std::io::Write,
{
    pub fn new(w: &'w mut W, limit: usize) -> Self {
        Self {
            w,
            allowed: limit,
            tab_replacement: DEFAULT_TAB_REPLACEMENT,
        }
    }
    pub const fn is_full(&self) -> bool {
        self.allowed == 0
    }
    /// return a tuple made of a cow containing either the given &str
    /// or the part fitting the remaining width, and the width of this cow
    pub fn cropped_str<'a>(&self, s: &'a str) -> (Cow<'a, str>, usize) {
        StrFit::make_cow(s, self.allowed)
    }
    pub fn queue_unstyled_str(&mut self, s: &str) -> Result<(), Error> {
        if self.is_full() {
            return Ok(());
        }
        let (string, len) = self.cropped_str(s);
        self.allowed -= len;
        self.w.queue(Print(string))?;
        Ok(())
    }
    pub fn queue_str(&mut self, cs: &CompoundStyle, s: &str) -> Result<(), Error> {
        if self.is_full() {
            return Ok(());
        }
        let (string, len) = self.cropped_str(s);
        self.allowed -= len;
        cs.queue(self.w, string)
    }
    /// Queue the char if it fits in the remaining width.
    /// A char which doesn't fit stops all further char writing.
    pub fn queue_char(&mut self, cs: &CompoundStyle, c: char) -> Result<(), Error> {
        if c == '\t' {
            return self.queue_str(cs, self.tab_replacement);
        }
        let width = UnicodeWidthChar::width(c).unwrap_or(0);
        if width > self.allowed {
            // a following narrower char must not take this char's place
            self.allowed = 0;
            return Ok(());
        }
        self.allowed -= width;
        cs.queue(self.w, c)?;
        Ok(())
    }
    /// Queue the char if it fits in the remaining width.
    /// A char which doesn't fit stops all further char writing.
    pub fn queue_unstyled_char(&mut self, c: char) -> Result<(), Error> {
        if c == '\t' {
            return self.queue_unstyled_str(self.tab_replacement);
        }
        let width = UnicodeWidthChar::width(c).unwrap_or(0);
        if width > self.allowed {
            // a following narrower char must not take this char's place
            self.allowed = 0;
            return Ok(());
        }
        self.allowed -= width;
        self.w.queue(Print(c))?;
        Ok(())
    }
    /// a "g_string" is a "gentle" one: each char takes one column on screen.
    /// This function must thus not be used for unknown strings.
    pub fn queue_unstyled_g_string(&mut self, mut s: String) -> Result<(), Error> {
        if self.is_full() {
            return Ok(());
        }
        let mut len = 0;
        for (idx, _) in s.char_indices() {
            len += 1;
            if len > self.allowed {
                s.truncate(idx);
                self.allowed = 0;
                self.w.queue(Print(s))?;
                return Ok(());
            }
        }
        self.allowed -= len;
        self.w.queue(Print(s))?;
        Ok(())
    }
    /// a "g_string" is a "gentle" one: each char takes one column on screen.
    /// This function must thus not be used for unknown strings.
    pub fn queue_g_string(&mut self, cs: &CompoundStyle, mut s: String) -> Result<(), Error> {
        if self.is_full() {
            return Ok(());
        }
        let mut len = 0;
        for (idx, _) in s.char_indices() {
            len += 1;
            if len > self.allowed {
                s.truncate(idx);
                self.allowed = 0;
                return cs.queue(self.w, s);
            }
        }
        self.allowed -= len;
        cs.queue(self.w, s)
    }
    pub fn queue_fg(&mut self, cs: &CompoundStyle) -> Result<(), Error> {
        cs.queue_fg(self.w)
    }
    pub fn queue_bg(&mut self, cs: &CompoundStyle) -> Result<(), Error> {
        cs.queue_bg(self.w)
    }
    pub fn fill(&mut self, cs: &CompoundStyle, filling: &'static Filling) -> Result<(), Error> {
        self.repeat(cs, filling, self.allowed)
    }
    pub fn fill_unstyled(&mut self, filling: &'static Filling) -> Result<(), Error> {
        self.repeat_unstyled(filling, self.allowed)
    }
    pub fn fill_with_space(&mut self, cs: &CompoundStyle) -> Result<(), Error> {
        self.repeat(cs, &SPACE_FILLING, self.allowed)
    }
    pub fn repeat(
        &mut self,
        cs: &CompoundStyle,
        filling: &'static Filling,
        mut len: usize,
    ) -> Result<(), Error> {
        len = len.min(self.allowed);
        self.allowed -= len;
        filling.queue_styled(self.w, cs, len)
    }
    pub fn repeat_unstyled(
        &mut self,
        filling: &'static Filling,
        mut len: usize,
    ) -> Result<(), Error> {
        len = len.min(self.allowed);
        self.allowed -= len;
        filling.queue_unstyled(self.w, len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn written(f: impl Fn(&mut CropWriter<'_, Vec<u8>>)) -> String {
        let mut buf = Vec::new();
        let mut cw = CropWriter::new(&mut buf, 5);
        f(&mut cw);
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn chars_fill_the_allowed_width() {
        let s = written(|cw| {
            for c in "abcdefg".chars() {
                cw.queue_unstyled_char(c).unwrap();
            }
        });
        assert_eq!(s, "abcde");
        let s = written(|cw| {
            for c in "abc".chars() {
                cw.queue_unstyled_char(c).unwrap();
            }
            cw.queue_unstyled_str("defg").unwrap();
        });
        assert_eq!(s, "abcde");
    }

    #[test]
    fn unfitting_char_stops_the_writing() {
        // the wide char doesn't fit in the last column, and 'e'
        // must not take its place
        let s = written(|cw| {
            for c in "abcd日e".chars() {
                cw.queue_unstyled_char(c).unwrap();
            }
        });
        assert_eq!(s, "abcd");
        let s = written(|cw| {
            for c in "abc日本".chars() {
                cw.queue_unstyled_char(c).unwrap();
            }
        });
        assert_eq!(s, "abc日");
    }

    #[test]
    fn combining_mark_kept_on_last_column() {
        let s = written(|cw| {
            for c in "abcde\u{301}".chars() {
                cw.queue_unstyled_char(c).unwrap();
            }
        });
        assert_eq!(s, "abcde\u{301}");
    }

    #[test]
    fn styled_chars_fill_the_allowed_width() {
        let cs = CompoundStyle::default();
        let s = written(|cw| {
            for c in "abcdefg".chars() {
                cw.queue_char(&cs, c).unwrap();
            }
            cw.fill(&cs, &SPACE_FILLING).unwrap();
        });
        assert_eq!(s, "abcde");
    }
}
