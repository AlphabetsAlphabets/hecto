use std::cmp;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

use super::editor::Position;


#[derive(Default)]
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

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.string[..].graphemes(true).skip(start).take(end - start) {
            // Change tabs to spaces
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn contents(&self) -> String {
        self.string.clone()
    }
}

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
    pub filename: String,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value.trim_end()));
        }

        let rows = Self {
            rows,
            filename: filename.to_string()
        };

        Ok(rows)
    }

    pub fn insert(&mut self,  c: char, at: &Position) {
        if let Some(current_line) = self.rows.get_mut(at.y) {
            let mut text: String = current_line.string.split_word_bounds().collect();
            text.push_str(&c.to_string());
            current_line.string = text;
            current_line.len += 1;
        }
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
