use std::fs;
use std::io::prelude::*;

use super::gap_buffer::GapBuffer;

pub struct Document {
    pub lines: Vec<GapBuffer>,
}

impl Document {
    pub fn open(file: &str) -> Self {
        let mut lines = vec![];
        let file_content = fs::read_to_string(file).unwrap();
        for line in file_content.lines() {
            lines.push(GapBuffer::from(line.to_string()));
        }

        Self { lines }
    }
}
