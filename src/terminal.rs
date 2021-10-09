use std::io::{self, stdout, Stdout, Write};

use super::editor::Position;

use crossterm::cursor;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{enable_raw_mode, size, Clear, ClearType};
use crossterm::{execute, queue, QueueableCommand};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = size().unwrap();
        let term = Self {
            size: Size {
                width: size.0.saturating_sub(1),
                height: size.1.saturating_sub(3),
            },
            stdout: stdout(),
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

    pub fn change_cursor_shape(&mut self, cursor_shape: cursor::CursorShape) {
        let cursor_shape = cursor::SetCursorShape(cursor_shape);
        execute!(self.stdout, cursor_shape).unwrap();
    }

    pub fn show_command_window(&mut self) {
        let doc_height = self.size.height as f32;
        let doc_width = self.size.width as f32;

        // x * n, if n is bigger moves cursor to right
        let x1 = (doc_width * 0.2) as u16;
        let x2 = (doc_width * 0.8) as u16;

        let mut y1 = (doc_height * 0.2) as u16;
        let y2 = (doc_height * 0.8) as u16;

        let hori_line = (x2 - x1) as usize;
        let vert_line = (y2 - y1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the top and bottom
        execute!(
            self.stdout,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2),
            Print(&hori_border),
        )
        .unwrap();

        y1 += 1;
        while y1 <= vert_line as u16 {
            execute!(self.stdout, cursor::MoveTo(x1, y1 as u16), Print("|")).unwrap();
            y1 += 1;
        }

        self.stdout.flush();
    }
}
