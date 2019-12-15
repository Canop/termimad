use crossterm;

/// a valid user event
#[derive(Debug, Clone)]
pub enum Event {

    Key(crossterm::event::KeyEvent),

    Click(u16, u16),

    RightClick(u16, u16),

    DoubleClick(u16, u16),

    /// terminal was resized. Contains the new dimensions
    Resize(u16, u16),

    /// mouse wheel turns. contains -1 if up or 1 if down
    Wheel(i32),
}

impl Event {
    /// convert a crossterm event into a termimad one.
    ///
    /// To get a double-click you'll either need to use a termimad event-source
    /// or to do the computation yourself.
    pub fn from_crossterm_event(
        crossterm_event: crossterm::Result<crossterm::event::Event>
    ) -> Option<Event> {
        match crossterm_event {
            Ok(crossterm::event::Event::Key(key)) => {
                Some(Event::Key(key))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::Up(button, x, y, ..))) => {
                match button {
                    crossterm::event::MouseButton::Left => Some(Event::Click(x, y)),
                    crossterm::event::MouseButton::Right => Some(Event::RightClick(x, y)),
                    _ => None
                }
            }
            Ok(crossterm::event::Event::Resize(w, h)) => {
                Some(Event::Resize(w, h))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::ScrollUp(..))) => {
                Some(Event::Wheel(-1))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::ScrollDown(..))) => {
                Some(Event::Wheel(1))
            }
            _ => None,
        }
    }
}

