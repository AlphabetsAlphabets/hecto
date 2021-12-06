use std::fs;
use std::io::prelude::*;

use super::editor::Position;
use super::rows::Row;

use unicode_segmentation::UnicodeSegmentation;

// TODO: Change rows to a gap buffer.

#[derive(Default, Clone)]
pub struct Document {
    pub rows: Vec<Row>,
    pub filename: String,
}

// Utility functions
impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value.trim_end()));
        }

        let rows = Self {
            rows,
            filename: filename.to_string(),
        };

        Ok(rows)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}

// Functions related to typing
impl Document {
    pub fn insert(&mut self, c: char, at: &Position) {
        if self.rows.is_empty() {
            let c = c.to_string();
            let row = Row::from(c.to_string().as_str());
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at, c);
        }
    }

    pub fn enter(&mut self, at: &Position) {
        self.insert_newline(at);
    }

    fn insert_newline(&mut self, at: &Position) {
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn delete(&mut self, at: &Position) {
        if let Some(current_row) = self.rows.get_mut(at.y) {
            if current_row.string.len() == 0 {
                // Removing rows, wemoving empty lines
                self.rows.remove(at.y);
            } else if at.x == 0 && at.y != 0 {
                // removing rows, appending the line below to the current line.
                let current_row = self.rows.get_mut(at.y).unwrap();
                let contents = current_row.contents();

                let mut row_above_current = self.rows.get_mut(at.y.saturating_sub(1)).unwrap();
                row_above_current.string = format!("{}{}", row_above_current.string, contents);

                self.rows.remove(at.y);
            } else {
                // removing the text from the row
                let current: String = current_row
                    .string
                    .graphemes(true)
                    .take(at.x.saturating_sub(1))
                    .collect();

                let remainder: String = current_row.string.graphemes(true).skip(at.x).collect();

                let new_row = format!("{}{}", current, remainder);
                let new_row = Row::from(new_row.as_str());
                *current_row = new_row;
            }
        }
    }

    fn truncate_and_open_file(&self) -> Result<fs::File, std::io::Error> {
        let mut file = fs::OpenOptions::new();
        file.write(true).truncate(true).open(&self.filename)
    }

    pub fn save_file(&mut self) {
        if let Ok(mut file) = self.truncate_and_open_file() {
            for row in &self.rows {
                let string = format!("{}\n", row.string);
                file.write_all(string.as_bytes()).unwrap();
            }
        }
    }
}
