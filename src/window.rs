use std::io::{StdoutLock, Write};

use crossterm::style::Print;
use crossterm::{cursor, queue};

use unicode_segmentation::UnicodeSegmentation;

use super::editor::Position;
use super::rows::Row;
use super::editor::Object;

#[derive(Clone, Default, Debug)]
pub struct Window {
    pub name: String,
    pub x1: u16, // left
    pub x2: u16, // right
    pub y1: u16, // up
    pub y2: u16, // down
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
            // rows: vec![],
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

    pub fn draw_border(&mut self, stdout: &mut StdoutLock, content: &Vec<String>) {
        let Self { x1, x2, y1, y2, .. } = *self;

        let halves = (x2 - x1 - self.name.len() as u16) as usize / 2;
        let left_half = "-".repeat(halves - 1);
        let right_half = "-".repeat(halves - 3);
        let top_border = format!("+{} {} {}+", left_half, self.name.to_uppercase(), right_half);

        let bottom_border = (x2 - x1) as usize;
        let border_fill = "-".repeat(bottom_border - 2);
        let bottom_border = format!("+{}+", border_fill);

        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&top_border),
            cursor::MoveTo(x1, y2 - 2),
            Print(&bottom_border),
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
                format!("|{}{}|", content.get(num).unwrap(), spaces)
            } else {
                let spaces = " ".repeat((x2 - x1 - 2).into());
                format!("|{}|", spaces)
            };

            queue!(stdout, cursor::MoveTo(x1, y as u16), Print(text)).unwrap();

            y += 1;
            num += 1;
        }

        queue!(stdout, cursor::Show).unwrap();
    }

    pub fn draw_text_box(&mut self, stdout: &mut StdoutLock, string: String) -> (u16, u16) {
        let Self { x1, x2, y2, .. } = *self;
        self.cur_pos_before = Position::from(self.get_cursor_position());

        let length_of_border = (x2 - x1) as usize;

        let repeat = (length_of_border - 2 - self.name.len()) / 2;
        let left_half = "-".repeat(repeat);
        let right_half = "-".repeat(repeat);


        // TODO: If string.len() == even, bar will shift to right or left.
        let top_half = format!("+{} {} {}+", left_half, string, right_half);
        let difference = (top_half.len() as isize - length_of_border as isize);

        let repeat = repeat as isize;
        let left_half = if difference > 0 {
            let repeat = (repeat - difference) as usize;
            "-".repeat(repeat)
        } else if difference < 0 {
            let repeat = (repeat + difference) as usize;
            "-".repeat(repeat)
        } else {
            "-".repeat(repeat as usize)
        };

        let top_half = format!("+{} {} {}+", left_half, string, right_half);

        let bottom_half = "-".repeat((x2 - x1 - 2).into());
        let bottom_half = format!("+{}+", bottom_half);

        let text_box = if let Some(text) = &self.string {
            // NOTE: This now instead makes the text box longer lmao.
            let max = (x2 - x1) as usize;
            let repeat = max - 5;
            let repeat = repeat.saturating_sub(text.len());

            let spaces = " ".repeat(repeat.into());
            // Object::log("text".to_string(), &text);
            Object::log("Text length".to_string(), &text.len());
            Object::log("max".to_string(), &max);

            if text.len() > max - 3 {
                // NOTE: The scrolling does work, but it's way too buggy.
                let mut text = text.clone();
                let diff: usize = text.len().saturating_sub(max);
                let _ = text.drain(..diff);
                format!("| {}", text)
            } else {
                format!("|-> {}{}|", text, spaces)
            }
        } else {
            let spaces = " ".repeat((x2 - x1 - 5).into());
            format!("|-> {}|", spaces)
        };

        // in y2 - X, increasing X moves it upwards.
        queue!(
            stdout,
            cursor::MoveTo(x1, y2 - 1),
            Print(&top_half),
            cursor::MoveTo(x1, y2 + 1),
            Print(&bottom_half),
            cursor::MoveTo(x1, y2),
            Print(text_box),
            cursor::MoveTo(x1 + 4, y2),
            cursor::Show,
        )
        .unwrap();

        self.get_cursor_position()
    }
}

impl Window {
    pub fn delete(&mut self, at: &Position) {
        if let Some(string) = &self.string {
            let mut text: String = string.clone().graphemes(true).rev().collect();
            text.remove(0);
            let text: String = text.graphemes(true).rev().collect();
            self.string = Some(text);
        }
    }
}
