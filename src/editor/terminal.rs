#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

pub struct Terminal {}

#[derive(Clone, Copy)]
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

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position { col: 0, row: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        Ok(Size {
            width: size()?.0 as usize,
            height: size()?.1 as usize,
        })
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
