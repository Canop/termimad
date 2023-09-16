use std::io::{stdout, Write};
use std::process::exit;
use std::thread::sleep;
use termimad::crossterm::{cursor, ExecutableCommand};
use termimad::{FmtText, MadSkin};
use termimad::crossterm::terminal::Clear;
use termimad::crossterm::terminal::ClearType::FromCursorDown;

pub struct IncrementalMarkdownPrinter {
    pub skin: MadSkin,
    pub wrap_width: Option<usize>,
    cursor_anchor: Option<(u16, u16)>,
    activated: bool,
}

impl IncrementalMarkdownPrinter {
    pub fn new(skin: MadSkin, wrap_width: Option<usize>) -> Self {
        IncrementalMarkdownPrinter {
            skin,
            wrap_width,
            cursor_anchor: None,
            activated: false,
        }
    }

    pub fn activated(&self) -> bool {
        self.activated
    }

    pub fn activate(&mut self) {
        if self.activated {
            eprintln!("IncrementalMarkdownPrinter is already activated");
            return;
        }
        self.activated = true;
        self.set_anchor();
        stdout().execute(cursor::Hide).unwrap();
    }

    pub fn set_anchor_with(&mut self, anchor_position: (u16, u16)) {
        assert!(self.activated, "IncrementalMarkdownPrinter must be activated before anchoring cursor");
        self.cursor_anchor = Some(anchor_position);
    }

    pub fn set_anchor(&mut self) {
        self.set_anchor_with(cursor::position().unwrap());
    }

    pub fn deactivate(&mut self) {
        if !self.activated {
            eprintln!("IncrementalMarkdownPrinter is already deactivated");
            return;
        }
        self.activated = false;
        self.cursor_anchor = None;
        stdout().execute(cursor::Show).unwrap();
    }

    pub fn print(&mut self, partial_markdown: &str) {
        assert!(self.activated, "IncrementalMarkdownPrinter must be activated before printing");
        let cursor_anchor = self.cursor_anchor.unwrap();
        // restore cursor position to anchor
        stdout()
            .execute(cursor::MoveTo(cursor_anchor.0, cursor_anchor.1)).unwrap()
            .execute(Clear(FromCursorDown)).unwrap(); // clear previous output
        let formatted_text = FmtText::from(&self.skin, partial_markdown, self.wrap_width.clone());
        let rows = formatted_text.lines.len() as u16;
        let columns = formatted_text.lines.last().as_ref().map_or(0, |last_line| last_line.visible_length()) as u16;
        print!("{}", formatted_text);
        stdout().flush().unwrap();
        // update cursor anchor
        // the cursor position is relative to the terminal not the screen/history, so the anchor "floats" when a scrollbar appears.
        let mut new_cursor_anchor = cursor::position().unwrap();
        if new_cursor_anchor.0 > columns {
            new_cursor_anchor.0 -= columns;
        } else {
            new_cursor_anchor.0 = 0;
        }
        if new_cursor_anchor.1 > rows {
            new_cursor_anchor.1 -= rows;
        } else {
            new_cursor_anchor.1 = 0;
        }
        self.set_anchor_with(new_cursor_anchor);
    }
}

impl Drop for IncrementalMarkdownPrinter {
    fn drop(&mut self) {
        if self.activated {
            self.deactivate();
        }
    }
}

const MARKDOWN_TEXT: &str = r#"
# Hello

This is inline code `print("hello")`.

A super long line to "tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttest" the hard wrapping.

```python
print("hello")
```

Here ends it."#;

fn main() {
    ctrlc::set_handler(move || {
        // to avoid missing cursor when Ctrl-C is pressed
        stdout().execute(cursor::Show).unwrap();
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    let skin = MadSkin::default();
    // let terminal_width = termimad::terminal_size().0 as usize;
    let terminal_width = 40; // for testing
    let mut printer = IncrementalMarkdownPrinter::new(skin, Some(terminal_width));
    printer.activate();
    let mut generator = MARKDOWN_TEXT.chars();
    let mut string_buffer = String::new();
    while let Some(chunk) = generator.next() {
        string_buffer.push(chunk);
        printer.print(string_buffer.as_str());
        sleep(std::time::Duration::from_millis(50));
    }
    printer.deactivate();
    sleep(std::time::Duration::from_millis(1000));
}