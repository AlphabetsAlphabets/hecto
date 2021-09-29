use std::cmp;
use std::fs;
use std::io::prelude::*;
use std::iter::FromIterator;
use unicode_segmentation::UnicodeSegmentation;

use super::editor::Position;

#[derive(Default, Clone)]
pub struct Row {
    pub string: String,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let mut row = Self {
            string: String::from(s),
            len: 0,
        };

        row.update_len();
        row
    }
}

impl From<String> for Row {
    fn from(s: String) -> Self {
        let mut row = Self { string: s, len: 0 };

        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            result.push_str(grapheme);
        }

        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len {
            self.string.push(c);
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn contents(&self) -> String {
        self.string.clone()
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len {
            return;
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let mut remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
            result.push_str(&remainder);
            self.string = result;
        }

        self.update_len();
    }
}

#[derive(Default)]
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
        if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn enter(&mut self, y: usize) {
        let mut new_row = Row::default();
        if y == self.rows.len() {
            self.rows.push(new_row);
        } else {
            let mut start = self.rows.iter().take(y + 1);
            let remainder = self.rows.iter().skip(y + 1);

            let mut rows: Vec<Row> = vec![];
            for row in start {
                let mut row = row.clone();
                row.string.push_str(&"\n".to_string());
                rows.push(row);
            }

            rows.push(new_row);
            for row in remainder {
                rows.push(row.clone());
            }

            self.rows = rows;
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }

        if at.x == self.rows.get_mut(at.y).unwrap().len && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
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
