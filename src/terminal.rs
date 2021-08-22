use std::io::{self, stdout, Write};

use super::editor::Position;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        let term = Self {
            size: Size {
                width: size.0 - 1,
                height: size.1,
            },
            stdout: stdout().into_raw_mode()?,
        };

        Ok(term)
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_current_line(&mut self) {
        write!(self.stdout, "{}", termion::clear::CurrentLine);
        self.stdout.flush().unwrap();
    }

    pub fn cursor_position(&mut self, pos: &Position) {
        // using `saturating_add` prevents the buffer from overflowing.
        let Position { mut x, mut y } = pos;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;

        write!(self.stdout, "{}", termion::cursor::Goto(x, y));
        self.stdout.flush().unwrap();
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
