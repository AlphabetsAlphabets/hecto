use std::cmp;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

use super::editor::Position;

#[derive(Default)]
pub struct Row {
    pub string: String,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self {
            string: String::from(s),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn contents(&self) -> String {
        self.string.clone()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value.trim_end()));
        }

        let rows = Self { rows };
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
