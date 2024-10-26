use {
    crate::crossterm::{
        self,
        event::{
            Event,
            KeyModifiers,
            MouseButton,
            MouseEvent,
            MouseEventKind,
        },
    },
    crokey::KeyCombination,
    std::time::Instant,
};

/// A user event based on a crossterm event, decorated
/// - with time
/// - with a double_click flag
/// - with a KeyCombination, if the event is a key ending a combination (which may be a simple key)
///
/// You normally don't build this yourself, but rather use
/// the [EventSource].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedEvent {
    pub time: Instant,

    pub event: crossterm::event::Event,

    /// false unless you set it yourself using the time
    /// or you get the timed event with an EventSource
    /// which computes it. Can be true only for left mouse
    /// down and left mouse up (both down and up of the second
    /// click have it true)
    pub double_click: bool,

    /// If you're interested in key combinations, you should
    /// prefer this field over the Key variant of the event
    /// field. If you want to react on Press or Repeat, then
    /// the event field holds the information.
    pub key_combination: Option<KeyCombination>,
}

impl TimedEvent {
    /// Wrap a crossterm event into a timed one, with time.
    ///
    /// You should normally not need to use this function, but rather obtain
    /// the timed event from an EventSource which build the normalized
    /// key combination, and sets the double_click flag.
    pub fn new(event: Event) -> Self {
        Self {
            time: Instant::now(),
            event,
            double_click: false,
            key_combination: None,
        }
    }

    /// If it's a simple mouse up and not determined to be the second click of
    /// a double click, return the coordinates
    pub fn as_click(&self) -> Option<(u16, u16)> {
        match self.event {
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Left),
                column,
                row,
                modifiers: KeyModifiers::NONE,
            }) if !self.double_click => Some((column, row)),
            _ => None,
        }
    }

    pub fn is_key<K: Into<KeyCombination>>(&self, key: K) -> bool {
        let key = key.into();
        if self.key_combination == Some(key) {
            return true;
        }
        if let Event::Key(k) = &self.event {
            let self_key: KeyCombination = (*k).into();
            if self_key == key {
                return true;
            }
        }
        false
    }
}
