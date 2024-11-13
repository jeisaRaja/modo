use crate::editor::{APP_NAME, VERSION};
use buffer::Buffer;
pub mod buffer;

use super::terminal::{Size, Terminal};

pub struct View {
    pub buffer: Buffer,
    size: Size,
    need_redraw: bool,
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
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
