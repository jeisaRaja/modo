use core::panic;
use crossterm::event::{read, Event};
use editor_command::EditorCommand;
use std::panic::{set_hook, take_hook};
pub mod terminal;
pub mod view;
use crate::Result;
use terminal::Terminal;
use view::View;
pub mod editor_command;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
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
                Ok(event) => self.evaluate_event(event),
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
        let _ = Terminal::move_caret_to(self.view.get_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn evaluate_event(&mut self, event: Event) {
        match EditorCommand::try_from(event) {
            Ok(command) => {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true
                } else {
                    self.view.handle_command(command);
                }
            }
            Err(err) => panic!("Could not process command: {err}"),
        }
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }
}
