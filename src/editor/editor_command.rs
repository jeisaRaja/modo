use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up | KeyCode::Char('k'), _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down | KeyCode::Char('j'), _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left | KeyCode::Char('h'), _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right | KeyCode::Char('l'), _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),

                _ => Err(format!("Key code not supported: {code:?}")),
            },
            Event::Resize(width_16, height_16) => {
                let height = height_16 as usize;
                let width = width_16 as usize;
                Ok(Self::Resize(Size { width, height }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
