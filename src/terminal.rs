use std::io::{Stdout, Write};

use super::editor::Position;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{enable_raw_mode, size, Clear, ClearType};

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        let prev_width = self.width;
        let prev_height = self.height;
        let prev = (prev_width, prev_height);

        let cur_width = other.width;
        let cur_height = other.height;
        let cur = (cur_width, cur_height);

        prev == cur
    }
}

impl PartialEq<(u16, u16)> for Size {
    /// other: (width, height)
    fn eq(&self, prev_dim: &(u16, u16)) -> bool {
        let cur_width = self.width;
        let cur_height = self.height;
        let cur_dim = (cur_width, cur_height);

        cur_dim == *prev_dim
    }
}

pub struct Terminal {
    size: Size,
    pub stdout: Stdout,
}

impl Terminal {
    pub fn new(stdout: Stdout) -> Result<Self, std::io::Error> {
        let size = size().unwrap();

        let term = Self {
            size: Size {
                width: size.0.saturating_sub(1),
                height: size.1.saturating_sub(3),
            },
            stdout,
        };

        enable_raw_mode().unwrap();
        Ok(term)
    }

    pub fn update_dimensions(&mut self, prev_dim: Size) {
        let cur_dim = size().unwrap();
        if prev_dim != cur_dim {
            self.clear_screen();
            self.size = Size {
                width: cur_dim.0.saturating_sub(1),
                height: cur_dim.1.saturating_sub(3),
            };
        }
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

    pub fn dim(&self) -> (u16, u16) {
        (self.size.width, self.size.height)
    }

    pub fn set_bg_color(&mut self, color: Color) {
        execute!(self.stdout, SetBackgroundColor(color)).unwrap();
    }

    pub fn set_fg_color(&mut self, color: Color) {
        execute!(self.stdout, SetForegroundColor(color)).unwrap();
    }

    pub fn reset_fg_color(&mut self) {
        execute!(self.stdout, SetForegroundColor(Color::Reset)).unwrap();
    }

    pub fn reset_bg_color(&mut self) {
        execute!(self.stdout, SetBackgroundColor(Color::Reset)).unwrap();
    }

    pub fn change_cursor_shape(&mut self, cursor_shape: cursor::CursorShape) {
        let cursor_shape = cursor::SetCursorShape(cursor_shape);
        execute!(self.stdout, cursor_shape).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        execute!(self.stdout, cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        execute!(self.stdout, cursor::Show).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
