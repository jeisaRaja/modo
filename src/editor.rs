use crossterm::event::{read, Event, Event::Key, KeyCode::*, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};
use std::cmp::min;
pub mod terminal;
pub mod view;
use crate::Result;
use terminal::{Size, Terminal};
use view::View;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<()> {
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

    fn refresh_screen(&mut self) -> Result<()> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(terminal::Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            self.view.render()?;
            Terminal::move_caret_to(terminal::Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<()> {
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

    fn move_to(&mut self, key_code: KeyCode) -> Result<()> {
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

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }
}
