use std::fs;
use std::io::prelude::*;

use super::gap_buffer::GapBuffer;
use super::editor::Position;

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
            let line = line.graphemes(true).collect::<String>();
            let buffer = GapBuffer::new(line.as_str());
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

    pub fn buffer_mut(&mut self, index: usize) -> Option<&mut GapBuffer> {
        self.gap_buffer.get_mut(index)
    }
}

// Functions related to typing
impl Document {
    pub fn insert(&mut self, c: char, at: &Position) {
        if let Some(buffer) = self.gap_buffer.get_mut(at.y) {
            buffer.insert(c, at.x);
        }
    }

    pub fn delete(&mut self, at: &Position) -> bool {
        if at.x == 0 && at.y != 0 {
            let mut current = self.gap_buffer.get(at.y).unwrap().clone();
            let mut above = self.gap_buffer.get(at.y - 1).unwrap().clone();

            above.chs.append(&mut current.chs);
            above.update_len();

            self.gap_buffer.remove(at.y);
            if let Some(old_above) = self.gap_buffer.get_mut(at.y - 1) {
                *old_above = above;
            }

            true
        } else {
            let current = self.gap_buffer.get_mut(at.y).unwrap();
            current.delete(at.x);

            false
        }
    }

    pub fn enter(&mut self, at: &Position) {
        let new = self.gap_buffer.get_mut(at.y).unwrap().split(at.x);
        self.gap_buffer.insert(at.y + 1, new);
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
