use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool
}

pub fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // This byte clears the screen
        print!("\x1b[2J");
        io::stdout().flush()
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                panic!(error);
            };

            if self.should_quit {
                break;
            };

            if let Err(error) = self.process_keypress() {
                panic!(error);
            };

        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}
