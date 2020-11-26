use {
    super::{
        Event,
        EscapeSequence,
    },
    crate::{
        errors::Error,
    },
    crossbeam::channel::{unbounded, Receiver, Sender},
    crossterm::{
        self,
        event::{
            KeyCode,
            KeyEvent,
            KeyModifiers,
        },
        terminal,
    },
    std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        thread,
        time::{Duration, Instant},
    }
};

const DOUBLE_CLICK_MAX_DURATION: Duration = Duration::from_millis(700);

struct TimedClick {
    time: Instant,
    x: u16,
    y: u16,
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
}

impl EventSource {
    /// create a new source
    ///
    /// If desired, mouse support must be enabled and disabled in crossterm.
    pub fn new() -> Result<EventSource, Error> {
        let (tx_events, rx_events) = unbounded();
        let (tx_quit, rx_quit) = unbounded();
        let event_count = Arc::new(AtomicUsize::new(0));
        let internal_event_count = Arc::clone(&event_count);
        terminal::enable_raw_mode()?;
        let mut last_click: Option<TimedClick> = None;
        let seq_start = KeyEvent {
            code: KeyCode::Char('_'),
            modifiers: KeyModifiers::ALT,
        };
        let seq_end = KeyEvent {
            code: KeyCode::Char('\\'),
            modifiers: KeyModifiers::ALT,
        };
        thread::spawn(move || {
            let mut current_escape_sequence: Option<EscapeSequence> = None;
            // return true when we must close the source
            let send_and_wait = |event| {
                if let Err(_) = tx_events.send(event) {
                    true // broken channel
                } else {
                    match rx_quit.recv() {
                        Ok(false) => false,
                        _ => true,
                    }
                }
            };
            loop {
                let ct_event = match crossterm::event::read() {
                    Ok(e) => e,
                    _ => { continue; }
                };
                internal_event_count.fetch_add(1, Ordering::SeqCst);
                let in_seq = current_escape_sequence.is_some();
                if in_seq {
                    if let crossterm::event::Event::Key(key) = ct_event {
                        if key == seq_end {
                            // it's a proper sequence ending, we send it as such
                            let mut seq = current_escape_sequence.take().unwrap();
                            seq.keys.push(key);
                            if send_and_wait(Event::EscapeSequence(seq)) {
                                return;
                            }
                            continue;
                        } else if !key.modifiers.intersects(KeyModifiers::ALT | KeyModifiers::CONTROL) {
                            // adding to the current escape sequence
                            current_escape_sequence.as_mut().unwrap().keys.push(key);
                            continue;
                        }
                    }
                    // it's neither part of a proper sequence, nor the end
                    // we send all previous events independently before sending this one
                    let seq = current_escape_sequence.take().unwrap();
                    for key in seq.keys {
                        if send_and_wait(Event::Key(key)) {
                            return;
                        }
                    }
                    // the current event will be sent normally
                } else if let crossterm::event::Event::Key(key) = ct_event {
                    if key == seq_start {
                        // starting a new sequence
                        current_escape_sequence = Some(EscapeSequence { keys: vec![key] });
                        continue;
                    }
                }
                if let Some(mut event) = Event::from_crossterm_event(ct_event) {
                    // save the event, and maybe change it
                    // (may change a click into a double-click)
                    if let Event::Click(x, y, ..) = event {
                        if let Some(TimedClick { time, x: last_x, y: last_y }) = last_click {
                            if
                                last_x == x && last_y == y
                                && time.elapsed() < DOUBLE_CLICK_MAX_DURATION
                            {
                                event = Event::DoubleClick(x, y);
                            }
                        }
                        last_click = Some(TimedClick { time: Instant::now(), x, y });
                    }
                    // we send the event to the receiver in the main event loop
                    if send_and_wait(event) {
                        return;
                    }
                }
            }
        });
        Ok(EventSource {
            rx_events,
            tx_quit,
            event_count,
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

impl Drop for EventSource {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
