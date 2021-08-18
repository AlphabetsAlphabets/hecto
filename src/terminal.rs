use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    size: (u16, u16),
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        let term = Self {
            size: (size.0, size.1),
            _stdout: stdout().into_raw_mode()?
        };

        Ok(term)
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
        io::stdout().flush();
    }

    pub fn refresh_screen() -> Result<(), std::io::Error> {
        // This byte clears the screen
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        Self::flush()
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x + 1;
        let y = y + 1;

        print!("{}", termion::cursor::Goto(x, y));
        Self::flush();
    }

    pub fn size(&self) -> (u16, u16) {
        (self.size.0, self.size.1)
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
