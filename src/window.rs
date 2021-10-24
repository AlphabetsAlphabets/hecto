use std::io::{Stdout, StdoutLock, Write};

use crossterm::{cursor, queue};
use crossterm::style::Print;
use crossterm::event::{Event, KeyCode as Key, KeyEvent, KeyModifiers as Mod};

use super::editor::Position;
use super::rows::Row;

#[derive(Clone, Default, Debug)]
pub struct Window {
    pub name: String,
    pub x1: u16,
    pub x2: u16,
    pub y1: u16,
    pub y2: u16,
    pub rows: Vec<Row>,
    pub cursor_position: Position,
    pub has_been_drawn: bool,
    pub has_content_changed: bool,
}

impl Window {
    /// Param order: x1, x2, y1, y2
    pub fn new(name: String, x1: u16, x2: u16, y1: u16, y2: u16) -> Self {
        Self {
            name,
            x1,
            x2,
            y1,
            y2,
            rows: vec![],
            cursor_position: Position { x: 4, y: 0 },
            has_been_drawn: false,
            has_content_changed: false
        }
    }

    fn draw_all(&self, stdout: &mut StdoutLock) {
        stdout.flush().unwrap();
    }

    pub fn draw_text_box(&mut self, stdout: &mut StdoutLock) {
        let Self { x1, x2, y1, y2, .. } = *self;
        let text_box_border = "-".repeat((x2 - x1 - 2).into());
        let text_entry_border = format!("+{}+", text_box_border);
        self.rows.push(Row::from(text_entry_border.clone().as_str()));

        let spaces = " ".repeat((x2 - x1 - 5).into());
        let text_box = format!("|-> {}|", spaces);

        self.rows.push(Row::from(text_box.as_str()));

        queue!(
            stdout,
            cursor::Show,
            cursor::MoveTo(x1, y2),
            Print(text_entry_border),
            cursor::MoveTo(x1, y2 - 1),
            Print(text_box),
            cursor::MoveTo(x1 + 4, y2 - 1),
        )
        .unwrap();
    }

    pub fn draw_border(&mut self, stdout: &mut StdoutLock) {
        let Self { x1, x2, y1, y2, .. } = *self;

        let hori_line = (x2 - x1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2 - 2),
            Print(&hori_border),
        )
        .unwrap();

        let mut y = y1 + 1;
        // TODO: Make this list come from somewhere else.
        let commands = vec!["Save file".to_string(), "Quit".to_string()];

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 - 2 {
            let repeat = if let Some(command) = commands.get(num) {
                command.len()
            } else {
                0
            } as u16;

            // results window
            let text = if num < commands.len() {
                let spaces = " ".repeat((x2 - x1 - repeat - 2).into());
                let row = format!("|{}{}|", commands.get(num).unwrap(), spaces);

                self.rows.push(Row::from(row.clone().as_str()));
                row
            } else {
                let spaces = " ".repeat((x2 - x1 - 2).into());
                let row = format!("|{}|", spaces);

                self.rows.push(Row::from(row.clone().as_str()));
                row
            };

            queue!(stdout, cursor::MoveTo(x1, y as u16), Print(text)).unwrap();

            y += 1;
            num += 1;
        }

        self.has_been_drawn = true;
        queue!(stdout, cursor::Show).unwrap();
    }

    pub fn draw_command_window(&mut self, stdout: &mut Stdout) {
        let Self { x1, x2, y1, y2, ..  } = *self;
        let hori_line = (x2 - x1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2 - 2),
            cursor::MoveTo(x1, y1),
        ).unwrap();

        let mut y = y1 + 1;
        let commands = vec!["Save file".to_string(), "Quit".to_string()];

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 - 2 {
            let repeat = if let Some(command) = commands.get(num) {
                command.len()
            } else {
                0
            } as u16;

            // results window
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

        self.has_been_drawn = true;
        queue!(stdout, cursor::Show).unwrap();
    }

    pub fn draw_window(&mut self, stdout: &mut StdoutLock) {
        let len = self.rows.len().saturating_sub(1);
        if !self.has_been_drawn {
            self.draw_border(stdout);
            self.draw_text_box(stdout);
        } else if self.has_content_changed {
            let text = self.rows.get_mut(len).unwrap();
            todo!("\n\n------\n\n THE TEXT ENTRY THING WORKED\n-----");
        }

        queue!(stdout, cursor::Show).unwrap();
        self.draw_all(stdout);
    }}
