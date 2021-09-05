//! This example demonstrates
//!  - a responsive layout
//!  - a scrollable markdown text
//!  - a single line input field
//!  - a password input
//!  - a textarea
//!  - handling key and mouse events
//!  - managing the focus of widgets
//!  - managing a terminal properly configured in "alternate" mode
//!  - logging events in a file (useful for event handling debugging)
//!
//! run this example with
//!   cargo run --example inputs
//!
//! if you want to have a log file generated, run
//!   TERMIMAD_LOG=debug cargo run --example inputs

#[macro_use]
extern crate cli_log;

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
            MouseEvent, MouseEventKind, MouseButton,
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

pub const CONTROL_C: KeyEvent = KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL };
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
    loop {
        // this simple application uses crossterm event source. If you need to deal
        // with your own events or do tasks in background, you might prefer to use
        // the termimad EventSource instead (see broot or bacon as examples).
        let redraw = match event::read() {
            Ok(Event::Key(key)) => {
                debug!("key event: {:?}", key);
                if key == CONTROL_Q || key == CONTROL_C {
                    break; // quit
                }
                view.apply_key_event(key);
                true
            }
            Ok(Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                modifiers: KeyModifiers::NONE,
            })) => {
                view.apply_click_event(column, row);
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
