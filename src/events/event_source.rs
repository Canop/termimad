use {
    super::{
        EscapeSequence,
        TimedEvent,
    },
    crate::{
        crossterm::{
            self,
            event::{
                Event,
                KeyCode,
                KeyEvent,
                KeyModifiers,
                MouseButton,
                MouseEvent,
                MouseEventKind,
            },
            terminal,
        },
        errors::Error,
    },
    crokey::Combiner,
    crossbeam_channel::{
        bounded,
        unbounded,
        Receiver,
        Sender,
    },
    std::{
        sync::{
            atomic::{
                AtomicUsize,
                Ordering,
            },
            Arc,
        },
        thread,
        time::{
            Duration,
            Instant,
        },
    },
};

const DOUBLE_CLICK_MAX_DURATION: Duration = Duration::from_millis(700);
const ESCAPE_SEQUENCE_CHANNEL_SIZE: usize = 10;

struct TimedClick {
    time: Instant,
    x: u16,
    y: u16,
}

pub struct EventSourceOptions {
    /// Whether to try combine key events into key combinations.
    /// This changes the behavior of the terminal, if it's compatible, then restores
    /// the standard behavior on drop.
    pub combine_keys: bool,
    /// When combining is enabled, you may either want "simple" keys
    /// (i.e. without modifier or space) to be handled on key press,
    /// or to wait for a key release so that maybe they may
    /// be part of a combination like 'a-b'.
    /// If combinations without modifier or space are unlikely in your
    /// application, you may make it feel snappier by setting this to true.
    ///
    /// This setting has no effect when combining isn't enabled.
    pub mandate_modifier_for_multiple_keys: bool,
    /// Whether to filter out raw key events (default true)
    /// (if you want to manage repeat, press, release, specifically, you're probably
    /// not interested in combining keys)
    pub discard_raw_key_events: bool,
    /// whether to filter out simple mouse moves (default true)
    pub discard_mouse_move: bool,
    /// whether to filter out mouse drag (default false)
    pub discard_mouse_drag: bool,
}

/// a thread backed event listener emmiting events on a channel.
///
/// The event source enables the terminal's raw mode and restores
///  it on drop
///
/// Additionnally to emmitting events, this source updates a
///  sharable event count, protected by an Arc. This makes
///  it easy for background computation to stop (or check if
///  they should) when a user event is produced.
///
/// The event source isn't tick based. It makes it possible to
/// built TUI with no CPU consumption while idle.
pub struct EventSource {
    is_combining_keys: bool,
    rx_events: Receiver<TimedEvent>,
    rx_seqs: Receiver<EscapeSequence>,
    tx_quit: Sender<bool>,
    event_count: Arc<AtomicUsize>,
}

impl Default for EventSourceOptions {
    fn default() -> Self {
        Self {
            combine_keys: false,
            mandate_modifier_for_multiple_keys: true,
            discard_raw_key_events: true,
            discard_mouse_move: true,
            discard_mouse_drag: false,
        }
    }
}

fn is_seq_start(key: KeyEvent) -> bool {
    key.code == KeyCode::Char('_') && key.modifiers == KeyModifiers::ALT
}
fn is_seq_end(key: KeyEvent) -> bool {
    key.code == KeyCode::Char('\\') && key.modifiers == KeyModifiers::ALT
}

impl EventSource {
    /// create a new source with default options
    ///
    /// If desired, mouse support must be enabled and disabled in crossterm.
    pub fn new() -> Result<Self, Error> {
        Self::with_options(EventSourceOptions::default())
    }
    /// return true if the source is configured to combine standard keys
    /// and the terminal supports it (it requires the 'kitty keyboard
    /// protocol').
    ///
    /// If true, you may receive events with multiple non-modifier keys,
    /// eg `ctrl-a-b`. If not, the same sequence of keys will be received
    /// as two successive combinations: `ctrl-a` and `ctrl-b`.
    ///
    /// Combining is not delay-based: you receive the combination as soon
    /// as the keys are released (or as soon as the key is pressed in
    /// most cases when `mandate_modifier_for_multiple_keys` is true).
    pub fn supports_multi_key_combinations(&self) -> bool {
        self.is_combining_keys
    }
    /// create a new source
    ///
    /// If desired, mouse support must be enabled and disabled in crossterm.
    pub fn with_options(options: EventSourceOptions) -> Result<Self, Error> {
        let mut combiner = Combiner::default();
        terminal::enable_raw_mode()?;
        let is_combining_keys = if options.combine_keys {
            combiner.enable_combining()?
        } else {
            false
        };
        combiner.set_mandate_modifier_for_multiple_keys(options.mandate_modifier_for_multiple_keys);
        let (tx_events, rx_events) = unbounded();
        let (tx_seqs, rx_seqs) = bounded(ESCAPE_SEQUENCE_CHANNEL_SIZE);
        let (tx_quit, rx_quit) = unbounded();
        let event_count = Arc::new(AtomicUsize::new(0));
        let internal_event_count = Arc::clone(&event_count);
        thread::spawn(move || {
            let mut last_up: Option<TimedClick> = None;
            let mut current_escape_sequence: Option<EscapeSequence> = None;
            // return true when we must close the source
            let send_and_wait = |event| {
                internal_event_count.fetch_add(1, Ordering::SeqCst);
                if tx_events.send(event).is_err() {
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
                    _ => {
                        continue;
                    }
                };
                let in_seq = current_escape_sequence.is_some();
                if in_seq {
                    if let crossterm::event::Event::Key(key) = ct_event {
                        if is_seq_end(key) {
                            // it's a proper sequence ending, we send it as such
                            let mut seq = current_escape_sequence.take().unwrap();
                            seq.keys.push(key);
                            if tx_seqs.try_send(seq).is_err() {
                                // there's probably just nobody listening on
                                // this zero size bounded channel
                            }
                            continue;
                        } else if !key
                            .modifiers
                            .intersects(KeyModifiers::ALT | KeyModifiers::CONTROL)
                        {
                            // adding to the current escape sequence
                            current_escape_sequence.as_mut().unwrap().keys.push(key);
                            continue;
                        }
                    }
                    // it's neither part of a proper sequence, nor the end
                    // we send all previous events independently before sending this one
                    let seq = current_escape_sequence.take().unwrap();
                    for key in seq.keys {
                        let mut timed_event = TimedEvent::new(Event::Key(key));
                        timed_event.key_combination = combiner.transform(key);
                        if options.discard_raw_key_events && timed_event.key_combination.is_none() {
                            continue;
                        }
                        if send_and_wait(timed_event) {
                            return;
                        }
                    }
                    // the current event will be sent normally
                } else if let crossterm::event::Event::Key(key) = ct_event {
                    if is_seq_start(key) {
                        // starting a new sequence
                        current_escape_sequence = Some(EscapeSequence { keys: vec![key] });
                        continue;
                    }
                }
                if let Event::Mouse(mouse_event) = ct_event {
                    if options.discard_mouse_move && mouse_event.kind == MouseEventKind::Moved {
                        continue;
                    }
                    if options.discard_mouse_drag
                        && matches!(mouse_event.kind, MouseEventKind::Drag(_))
                    {
                        continue;
                    }
                }
                let mut timed_event = TimedEvent::new(ct_event);
                if let Event::Key(key) = &timed_event.event {
                    timed_event.key_combination = combiner.transform(*key);
                    if options.discard_raw_key_events && timed_event.key_combination.is_none() {
                        continue;
                    }
                }
                if let Event::Mouse(MouseEvent {
                    kind, column, row, ..
                }) = timed_event.event
                {
                    if matches!(
                        kind,
                        MouseEventKind::Down(MouseButton::Left)
                            | MouseEventKind::Up(MouseButton::Left)
                    ) {
                        if let Some(TimedClick { time, x, y }) = last_up {
                            if column == x
                                && row == y
                                && timed_event.time - time < DOUBLE_CLICK_MAX_DURATION
                            {
                                timed_event.double_click = true;
                            }
                        }
                        if kind == MouseEventKind::Up(MouseButton::Left) {
                            last_up = Some(TimedClick {
                                time: timed_event.time,
                                x: column,
                                y: row,
                            });
                        }
                    }
                }
                // we send the event to the receiver in the main event loop
                if send_and_wait(timed_event) {
                    return;
                }
            }
        });
        Ok(EventSource {
            is_combining_keys,
            rx_events,
            rx_seqs,
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
    pub fn receiver(&self) -> Receiver<TimedEvent> {
        self.rx_events.clone()
    }

    /// return a new receiver for the channel emmiting escape sequences
    ///
    /// It's a bounded channel and any escape sequence will be
    /// dropped when it's full
    pub fn escape_sequence_receiver(&self) -> Receiver<EscapeSequence> {
        self.rx_seqs.clone()
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
