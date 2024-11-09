use std::fs;

use crate::editor::{APP_NAME, VERSION};
use crate::Result;
use buffer::Buffer;
pub mod buffer;

use super::terminal::{Size, Terminal};

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = self.buffer.load(filename) {
            self.buffer = buffer;
        }
    }

    pub fn render(&self) -> Result<()> {
        if self.buffer.is_empty() {
            Self::render_welcome_message()
        } else {
            self.render_buffer()
        }
    }

    pub fn render_buffer(&self) -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(cur_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn render_welcome_message() -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            if cur_row == height / 3 {
                Self::draw_welcome_text()?;
            } else {
                Self::draw_empty_row()?;
            }
            if cur_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row() -> Result<()> {
        Terminal::print("~")
    }

    fn draw_welcome_text() -> Result<()> {
        let mut welcome_message = format!("{APP_NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;

        Ok(())
    }
}
