use std::io::{self, stdout, Write, Stdout};

use super::editor::Position;

use crossterm::execute;
use crossterm::cursor;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: Stdout
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        let term = Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            stdout: stdout()
        };

        enable_raw_mode().unwrap();
        Ok(term)
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", Clear(ClearType::All)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_current_line(&mut self) {
        write!(self.stdout, "{}", Clear(ClearType::CurrentLine)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_position(&mut self, pos: &Position) {
        let Position { x, y } = pos;
        let x = *x as u16;
        let y = *y as u16;

        write!(self.stdout, "{}", cursor::MoveTo(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_bg_color(&mut self, color: Color) {
        execute!(self.stdout, SetBackgroundColor(color)).unwrap();
    }

    pub fn set_fg_color(&mut self, color: Color) {
        execute!(self.stdout, SetForegroundColor(color));
    }

    pub fn reset_fg_color(&mut self) {
        execute!(self.stdout, SetForegroundColor(Color::Reset));
    }

    pub fn reset_bg_color(&mut self) {
        execute!(self.stdout, SetBackgroundColor(Color::Reset));
    }
}
