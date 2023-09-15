use std::io::{stdout, Write};
use std::thread::sleep;
use termimad::crossterm::{cursor, ExecutableCommand};
use termimad::{FmtText, MadSkin};
use termimad::crossterm::terminal::Clear;
use termimad::crossterm::terminal::ClearType::FromCursorDown;

const MARKDOWN_TEXT: &str = r#"
# Hello

This is inline code `print("hello")`.

```python
print("hello")
```

Here ends it.
"#;


fn main() {
    let skin = MadSkin::default();
    stdout().execute(cursor::Hide).unwrap();  // Hide cursor to hide cursor movement
    let mut string_buffer = String::new();
    let mut generator = MARKDOWN_TEXT.chars();
    stdout().execute(cursor::SavePosition).unwrap();  // save initial cursor position
    loop {
        let next_one = generator.next();
        if let Some(chunk) = next_one {
            stdout()
                .execute(cursor::RestorePosition).unwrap() // restore cursor position to initial
                .execute(Clear(FromCursorDown)).unwrap(); // clear previous output
            string_buffer.push(chunk);
            let formatted_text = FmtText::from(&skin, &string_buffer, None); // can have Some(width) to enable hard wrapping
            print!("{}", formatted_text);
            stdout().flush().unwrap();
            sleep(std::time::Duration::from_millis(100));
        } else {
            break;
        }
    }
    stdout().execute(cursor::Show).unwrap();
    sleep(std::time::Duration::from_millis(500));
}