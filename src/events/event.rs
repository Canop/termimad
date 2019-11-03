use crossterm::input::{InputEvent, KeyEvent, MouseButton, MouseEvent};

/// a valid user event
#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Click(u16, u16),
    DoubleClick(u16, u16),
    Wheel(i32),
}

impl Event {
    pub fn from_crossterm_event(crossterm_event: Option<InputEvent>) -> Option<Event> {
        match crossterm_event {
            Some(InputEvent::Keyboard(key)) => Some(Event::Key(key)),
            Some(InputEvent::Mouse(MouseEvent::Release(x, y))) => Some(Event::Click(x, y)),
            Some(InputEvent::Mouse(MouseEvent::Press(MouseButton::WheelUp, _, _))) => {
                Some(Event::Wheel(-1))
            }
            Some(InputEvent::Mouse(MouseEvent::Press(MouseButton::WheelDown, _, _))) => {
                Some(Event::Wheel(1))
            }
            _ => None,
        }
    }
}
