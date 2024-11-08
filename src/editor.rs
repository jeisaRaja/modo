use crossterm::event::{read, Event, Event::Key, KeyCode::*, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};
use std::cmp::min;
use std::io::{stdout, Error, Write};
pub mod terminal;
use terminal::{Size, Terminal};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            location: Location { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(terminal::Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(terminal::Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                Up | Down | Left | Right => self.move_to(*code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn move_to(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match key_code {
            Up => y = y.saturating_sub(1),
            Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            Left => x = x.saturating_sub(1),
            Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            _ => (),
        }

        self.location = Location { x, y };
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            if cur_row == height / 3 {
                Self::draw_welcome_text()?;
            } else {
                Self::draw_empty_row()?;
            }
            if cur_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }

    fn draw_welcome_text() -> Result<(), Error> {
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
