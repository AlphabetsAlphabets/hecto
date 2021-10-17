use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

use super::editor::Position;

#[derive(Default, Clone, Debug)]
pub struct Row<'a> {
    pub string: &'a str,
    pub len: usize,
}

impl<'a> From<&str> for Row<'a> {
    fn from(s: &'a str) -> Self {
        let mut row = Self {
            string: s,
            len: 0,
        };

        row.update_len();
        row
    }
}

// Utility
impl<'a> Row<'a> {
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

    /// Splits current row into two rows, first half is before at, second half is after at.
    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string.graphemes(true).take(at).collect();
        let remainder: String = self.string.graphemes(true).skip(at).collect();

        self.string = &beginning;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn contents(&self) -> String {
        self.string.clone().to_string()
    }
}

// Text related
impl<'a> Row<'a> {
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.x >= self.len {
            let mut string = self.string.clone().to_string();
            string.push(c);
            self.string = &string;
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at.x).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at.x).collect();
            result.push(c);
            result.push_str(&remainder);
            self.string = &result;
        }

        self.update_len();
    }
}
