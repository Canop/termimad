use crossbeam::channel::{unbounded, Receiver, Sender};
use crossterm::{RawScreen, TerminalInput};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

use crate::{errors::Error, events::Event};

const DOUBLE_CLICK_MAX_DURATION: Duration = Duration::from_millis(700);

/// an event with time of occuring
struct TimedEvent {
    time: Instant,
    event: Event,
}
impl From<Event> for TimedEvent {
    fn from(event: Event) -> Self {
        TimedEvent {
            time: Instant::now(),
            event,
        }
    }
}

/// a thread backed event listener emmiting events on a channel.
///
/// Additionnally to emmitting events, this source updates a
///  sharable event count, protected by an Arc. This makes
///  it easy for background computation to stop (or check if
///  they should) when a user event is produced.
pub struct EventSource {
    rx_events: Receiver<Event>,
    tx_quit: Sender<bool>,
    event_count: Arc<AtomicUsize>,
    _raw_screen: RawScreen, // leaves raw screen on drop
}

impl EventSource {
    /// create a new source
    ///
    /// If desired, mouse support must be separately
    /// enabled:
    /// ```
    /// use crossterm::{EnableMouseCapture, queue};
    /// # use std::io::Write;
    /// # let mut w = std::io::stdout();
    /// queue!(w, EnableMouseCapture).unwrap();
    ///
    /// ```
    /// and disabled:
    /// ```
    /// use crossterm::{DisableMouseCapture, queue};
    /// # use std::io::Write;
    /// # let mut w = std::io::stdout();
    /// queue!(w, DisableMouseCapture).unwrap();
    ///
    /// ```
    pub fn new() -> Result<EventSource, Error> {
        let (tx_events, rx_events) = unbounded();
        let (tx_quit, rx_quit) = unbounded();
        let event_count = Arc::new(AtomicUsize::new(0));
        let internal_event_count = Arc::clone(&event_count);
        let _raw_screen = RawScreen::into_raw_mode()?;
        let input = TerminalInput::new();
        let mut last_event: Option<TimedEvent> = None;
        let mut crossterm_events = input.read_sync();
        thread::spawn(move || {
            loop {
                let crossterm_event = crossterm_events.next();
                if let Some(mut event) = Event::from_crossterm_event(crossterm_event) {
                    // save the event, and maybe change it
                    // (may change a click into a double-click)
                    if let Event::Click(x, y) = event {
                        if let Some(TimedEvent {
                            time,
                            event: Event::Click(_, last_y),
                        }) = last_event
                        {
                            if last_y == y && time.elapsed() < DOUBLE_CLICK_MAX_DURATION {
                                event = Event::DoubleClick(x, y);
                            }
                        }
                    }
                    last_event = Some(TimedEvent::from(event.clone()));
                    internal_event_count.fetch_add(1, Ordering::SeqCst);
                    // we send the even to the receiver in the main event loop
                    tx_events.send(event).unwrap();
                    let quit = rx_quit.recv().unwrap();
                    if quit {
                        // Cleanly quitting this thread is necessary
                        //  to ensure stdin is properly closed when
                        //  we launch an external application in the same
                        //  terminal
                        // Disabling mouse mode is also necessary to let the
                        //  terminal in a proper state.
                        input.disable_mouse_mode().unwrap();
                        return;
                    }
                }
            }
        });
        Ok(EventSource {
            rx_events,
            tx_quit,
            event_count,
            _raw_screen,
        })
    }

    /// either start listening again, or quit, depending on the passed bool.
    /// It's mandatory to call this with quit=true at end for a proper ending
    /// of the thread (and its resources)
    pub fn unblock(&self, quit: bool) {
        self.tx_quit.send(quit).unwrap();
    }

    /// return a shared reference to the event count. Other threads can
    ///  use it to check whether something happened (when there's no
    ///  parallel computation, the event channel is usually enough).
    pub fn shared_event_count(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.event_count)
    }

    /// return a new receiver for the channel emmiting events
    pub fn receiver(&self) -> Receiver<Event> {
        self.rx_events.clone()
    }
}
