use std::io::{StdoutLock, Write};

use crossterm::style::Print;
use crossterm::{cursor, queue};

use super::editor::Position;
use super::rows::Row;

#[derive(Clone, Default, Debug)]
pub struct Window {
    pub name: String,
    pub x1: u16, // left
    pub x2: u16, // right
    pub y1: u16, // up
    pub y2: u16, // down
    pub rows: Vec<Row>,
    /// For typing
    pub cursor_position: Position,
    pub string: Option<String>,
    pub cur_pos_before: Position,
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
            cursor_position: Position { x: 0, y: 0 },
            string: None,
            cur_pos_before: Position { x: 0, y: 0 },
        }
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        cursor::position().unwrap()
    }

    pub fn draw_all(&self, stdout: &mut StdoutLock) {
        stdout.flush().unwrap();
    }

    pub fn draw_text_box(&mut self, stdout: &mut StdoutLock, string: Option<String>) -> (u16, u16) {
        self.cur_pos_before = Position::from(self.get_cursor_position());

        let Self { x1, x2, y2, .. } = *self;
        let text_entry_border = if let Some(string) = string {
            let repeat = (x2 - x1 - 2 - self.name.len() as u16) as usize / 2;
            let left_half = "-".repeat(repeat);
            let right_half = "-".repeat(repeat);
            todo!("This needs fixing. Because depending on the word, the bar will shift.");
            format!("+{}{}{}+", left_half, string, right_half)
        } else {
            let text_box_border = "-".repeat((x2 - x1 - 2).into());
            format!("+{}+", text_box_border)
        };

        self.rows.push(Row::from(text_entry_border.as_str()));

        let text_box = if let Some(text) = &self.string {
            // NOTE: This now instead makes the text box longer lmao.
            let repeat = x2.saturating_sub(x1).saturating_sub(text.len() as u16).saturating_sub(5).into();
            let spaces = " ".repeat(repeat);
            format!("|-> {}{}|", text, spaces)
        } else {
            let spaces = " ".repeat((x2 - x1 - 5).into());
            format!("|-> {}|", spaces)
        };

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

        self.get_cursor_position()
    }

    pub fn draw_border(&mut self, stdout: &mut StdoutLock, content: &Vec<String>) {
        let Self { x1, x2, y1, y2, .. } = *self;

        let halves = (x2 - x1 - self.name.len() as u16) as usize / 2;
        let left_half = "-".repeat(halves - 1);
        let right_half = "-".repeat(halves - 3);
        let top_border = format!("+{} {} {}+", left_half, self.name.to_uppercase(), right_half);

        let hori_line = (x2 - x1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);
        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&top_border),
            cursor::MoveTo(x1, y2 - 2),
            Print(&hori_border),
        )
        .unwrap();

        let mut y = y1 + 1;
        // TODO: Make this list come from somewhere else.

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 - 2 {
            let repeat = if let Some(command) = content.get(num) {
                command.len()
            } else {
                0
            } as u16;

            // results window
            let text = if num < content.len() {
                let spaces = " ".repeat((x2 - x1 - repeat - 2).into());
                let row = format!("|{}{}|", content.get(num).unwrap(), spaces);

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

        queue!(stdout, cursor::Show).unwrap();
    }
}
