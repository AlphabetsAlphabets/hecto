use std::fs;
use std::io::prelude::*;

use super::editor::Position;
use super::rows::Row;

use unicode_segmentation::UnicodeSegmentation;

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
        let row = self.rows.get_mut(at.y).unwrap();
        row.insert(at, c);
    }

    pub fn enter(&mut self, at: &Position) {
        self.insert_newline(at);
    }

    fn insert_newline(&mut self, at: &Position) {
        // NOTE: This part is supposed to let you split the line into two halves
        // and have the right half of the line move underneath.
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn delete(&mut self, at: &Position) {
        let mut current_row = self.rows.get_mut(at.y).unwrap();

        if current_row.string.len() == 0 {
            let mut start = self.rows
                .get_mut(..at.y.saturating_sub(1))
                .unwrap()
                .to_vec();

            let mut remainder = self.rows.get_mut(at.y..).unwrap().to_vec();
            let mut rows: Vec<Row> = vec![];

            rows.append(&mut start);
            rows.append(&mut remainder);
        } else {
            let current: String = current_row
                .string
                .graphemes(true)
                .take(at.x.saturating_sub(1))
                .collect();

            let remainder: String = current_row.string.graphemes(true).skip(at.x).collect();

            let new_row = format!("{}{}", current, remainder);
            let new_row = Row::from(new_row);
            *current_row = new_row;
        }
    }

    pub fn save_file(&mut self) {
        let mut file = fs::OpenOptions::new();
        let file = file.truncate(true).write(true).open(&self.filename);
        if let Ok(mut file) = file {
            for row in &self.rows {
                if file.write_all(row.string.as_bytes()).is_err() {
                    eprintln!("CANNOT SAVE.");
                }
            }
        } else {
            eprintln!("CANNOT SAVE. NO FILE IS OPENED.");
        }
    }
}
