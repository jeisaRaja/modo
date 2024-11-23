use core::panic;
use crossterm::event::{read, Event, KeyCode::*, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};
use std::cmp::min;
use std::panic::{set_hook, take_hook};
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

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}

impl Editor {
    pub fn new() -> Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let view = View::default();

        let mut editor = Self {
            should_quit: false,
            location: Location::default(),
            view,
        };

        editor.handle_args();
        Ok(editor)
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}")
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        let _ = Terminal::move_caret_to(terminal::Position::default());
        self.view.render();
        let _ = Terminal::move_caret_to(terminal::Position {
            col: self.location.x,
            row: self.location.y,
        });

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn evaluate_event(&mut self, event: &Event) {
        match *event {
            Event::Resize(width, height) => {
                let height = height as usize;
                let width = width as usize;
                self.view.resize(Size { width, height });
            }
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                Up | Down | Left | Right | Char('j') | Char('k') | Char('h') | Char('l') => {
                    self.move_to(code)
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn move_to(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match key_code {
            Up | Char('k') => y = y.saturating_sub(1),
            Down | Char('j') => y = min(height.saturating_sub(1), y.saturating_add(1)),
            Left | Char('h') => x = x.saturating_sub(1),
            Right | Char('l') => x = min(width.saturating_sub(1), x.saturating_add(1)),
            _ => (),
        }

        self.location = Location { x, y };
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }
}
