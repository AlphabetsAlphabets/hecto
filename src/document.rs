use std::fs;
use std::io::prelude::*;

use super::gap_buffer::GapBuffer;
use super::editor::Position;
use super::rows::Row;

use unicode_segmentation::UnicodeSegmentation;

// TODO: Change rows to a gap buffer.
#[derive(Default, Clone)]
pub struct Document {
    pub gap_buffer: Vec<GapBuffer>,
    pub filename: String,
}

// Utility functions
impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut gap_buffer = vec![];
        for line in contents.lines() {
            let buffer = GapBuffer::new(line);
            gap_buffer.push(buffer);
        }

        let filename = filename.to_string();

        Ok(Self { gap_buffer, filename })
    }

    pub fn is_empty(&self) -> bool {
        self.gap_buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        self.gap_buffer.len()
    }

    pub fn buffer(&self, index: usize) -> Option<&GapBuffer> {
        self.gap_buffer.get(index)
    }
}

// Functions related to typing
impl Document {
    pub fn insert(&mut self, c: char, at: &Position) {
        if let Some(buffer) = self.gap_buffer.get_mut(at.y) {
            buffer.insert(c, at.x);
        }
    }

    pub fn enter(&mut self, at: &Position) {
        self.insert_newline(at);
    }

    fn insert_newline(&mut self, at: &Position) {
    }

    pub fn delete(&mut self, at: &Position) {
    }

    fn truncate_and_open_file(&self) -> Result<fs::File, std::io::Error> {
        let mut file = fs::OpenOptions::new();
        file.write(true).truncate(true).open(&self.filename)
    }

    pub fn save_file(&mut self) {
        if let Ok(mut file) = self.truncate_and_open_file() {
        }
    }
}
