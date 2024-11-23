use crate::editor::terminal::Position;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Position {
            col: location.x,
            row: location.y,
        }
    }
}
