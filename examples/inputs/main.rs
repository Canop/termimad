//! run this example with
//!   cargo run --example inputs
//!
//! if you want to have a log file generated, run
//!   TERMIMAD_LOG=debug cargo run --example inputs

#[macro_use]
extern crate cli_log;

mod clipboard;
mod view;

use {
    anyhow::{self},
    crossterm::{
        cursor,
        event::{
            self,
            DisableMouseCapture, EnableMouseCapture,
            Event,
            KeyCode, KeyEvent, KeyModifiers,
            MouseEvent, MouseEventKind,
        },
        terminal::{
            self,
            EnterAlternateScreen, LeaveAlternateScreen,
        },
        QueueableCommand,
    },
    std::io::{stdout, Write},
    termimad::*,
};

// Those 2 lines will be usable when crossterm merges KeyEvent::new being made const
// pub const CONTROL_C: KeyEvent = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
// pub const CONTROL_Q: KeyEvent = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);

pub const CONTROL_Q: KeyEvent = KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL };

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    // here we use stdout, we could also have used stderr, each has its benefits:
    // - using stdout provides buffering (hence the flushes) which means there's no
    //     flickering when we clear the whole screen before drawing
    // - using stderr allows piping data into another program with stdout
    let mut w = stdout();
    w.queue(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    w.queue(cursor::Hide)?;
    w.queue(EnableMouseCapture)?;
    let res = run_in_alternate(&mut w);
    w.queue(DisableMouseCapture)?;
    w.queue(cursor::Show)?; // we must restore the cursor
    terminal::disable_raw_mode()?;
    w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    res
}

/// run the event loop, in a terminal which must be in alternate
fn run_in_alternate<W: Write>(w: &mut W) -> anyhow::Result<()> {
    let mut view = view::View::new(Area::full_screen());
    view.queue_on(w)?;
    w.flush()?;
    // the clipboard backend depends on the system and varies in
    // capacity
    info!("clipboard backend type: {}", terminal_clipboard::get_type());
    loop {
        // this simple application uses crossterm event source. If you need to deal
        // with your own events or do tasks in background, you might prefer to use
        // the termimad EventSource instead (see broot or bacon as examples).
        let event = event::read();
        //debug!("event: {:?}", event);
        let redraw = match event {
            Ok(Event::Key(key)) => {
                debug!("key event: {:?}", key);
                if key == CONTROL_Q {
                    break; // quit
                }
                view.apply_key_event(key);
                true
            }
            Ok(Event::Mouse(MouseEvent { kind: MouseEventKind::Moved, .. })) => {
                // we don't want to refresh the terminal on each mouse move
                false
            }
            Ok(Event::Mouse(mouse_event)) => {
                view.apply_mouse_event(mouse_event);
                true
            }
            Ok(Event::Resize(width, height)) => {
                view.resize(Area::new(0, 0, width, height));
                true
            }
            _ => {
                false
            }
        };
        if redraw {
            view.queue_on(w)?;
            w.flush()?;
        }
    }
    Ok(())
}
