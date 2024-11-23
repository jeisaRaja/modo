#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
use crate::Result;
use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub struct Terminal {}

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn default() -> Self {
        Position { col: 0, row: 0 }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = Self::terminate();
    }
}

impl Terminal {
    pub fn terminate() -> Result<()> {
        Self::execute()?;
        disable_raw_mode()?;
        Self::leave_alternate_screen()?;
        Ok(())
    }

    pub fn initialize() -> Result<()> {
        Self::enter_alternate_screen()?;
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position { col: 0, row: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<()> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<()> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<()> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<()> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn move_caret_to(pos: Position) -> Result<()> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size> {
        Ok(Size {
            width: size()?.0 as usize,
            height: size()?.1 as usize,
        })
    }

    pub fn print(string: &str) -> Result<()> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn print_line(cur_row: usize, string: &str) -> Result<()> {
        _ = Self::move_caret_to(Position {
            col: 0,
            row: cur_row,
        });
        Self::clear_line()?;
        Self::print(string)?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<()> {
        queue!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<()> {
        queue!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn execute() -> Result<()> {
        stdout().flush()?;
        Ok(())
    }
}
