use crate::editor::{APP_NAME, VERSION};
use buffer::Buffer;
pub mod buffer;
pub mod location;
use location::Location;

use super::{
    editor_command::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

pub struct View {
    pub buffer: Buffer,
    size: Size,
    need_redraw: bool,
    location: Location,
    scroll_offset: Location,
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.need_redraw = true;
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = self.buffer.load(filename) {
            self.buffer = buffer;
        }
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }
        let Size { width, height } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        for cur_row in 0..height {
            if let Some(line) = self.buffer.lines.get(cur_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(cur_row, truncated_line);
            } else if cur_row == (height / 3) && self.buffer.is_empty() {
                Self::render_line(cur_row, &Self::build_welcome_message(self.size.width));
            } else {
                Self::render_line(cur_row, "~");
            }
        }
        self.need_redraw = false;
    }

    pub fn render_line(cur_row: usize, text: &str) {
        let result = Terminal::print_line(cur_row, text);
        debug_assert!(result.is_ok(), "Error when rendering line.");
    }

    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let welcome_message = format!("{APP_NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = width.saturating_sub(len).saturating_sub(1) / 2;
        let full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message
    }
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(dir) => self.move_text_location(&dir),
            EditorCommand::Quit => {}
            EditorCommand::Resize(size) => self.resize(size),
        }
    }

    fn move_text_location(&mut self, dir: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = self.size;
        match dir {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::PageUp => y = 0,
            Direction::PageDown => y = height.saturating_sub(1),
            Direction::Left => x = x.saturating_sub(1),
            Direction::Right => x = x.saturating_add(1),
            Direction::Home => x = 0,
            Direction::End => x = width.saturating_sub(1),
        }

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.need_redraw = offset_changed;
    }

    pub fn get_position(&self) -> Position {
        Position::from(self.location)
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
