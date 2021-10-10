use std::io::{Stdout, Write};

use crossterm::cursor;
use crossterm::style::Print;
use crossterm::queue;

use super::rows::Row;

pub struct Window {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
    rows: Vec<Row>,
}

impl Window {
    /// Param order: x1, x2, y1, y2
    pub fn new(x1: u16, x2: u16, y1: u16, y2: u16) -> Self {
        Self {
            x1,
            x2,
            y1,
            y2,
            rows: vec![],
        }
    }

    pub fn draw_all(&self, stdout: &mut Stdout) {
        stdout.flush().unwrap();
    }

    pub fn draw_command_window(&mut self, stdout: &mut Stdout) {
        let Self { x1, x2, y1, y2, ..  } = *self;

        let hori_line = (x2 - x1) as usize;
        let vert_line = (y2 - y1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the horizontall top and bottom walls
        queue!(
            stdout,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2),
            Print(&hori_border)
        ).unwrap();

        let mut y = y1 + 1;
        let commands = vec!["Save file".to_string(), "Quit".to_string()];

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 {
            let repeat = if let Some(command) = commands.get(num) {
                command.len()
            } else {
                0
            } as u16;

            let text = if num < commands.len() {
                let spaces = " ".repeat((x2 - x1 - repeat - 2).into());
                format!("{}{}", commands.get(num).unwrap(), spaces)
            } else {
                let spaces = " ".repeat((x2 - x1 - 2).into());
                format!("{}", spaces)
            };

            queue!(
                stdout,
                cursor::MoveTo(x1, y as u16),
                Print("|"),
                cursor::MoveTo(x1 + 1, y as u16),
                Print(text),
                Print("|"),
            ).unwrap();

            y += 1;
            num += 1;
        }
    }
}
