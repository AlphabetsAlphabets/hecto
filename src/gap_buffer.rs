use std::fmt;
use std::cmp;

use unicode_segmentation::UnicodeSegmentation;
use super::editor::Position;
use std::io::prelude::*;

#[derive(Default, Clone)]
pub struct GapBuffer {
    pub cur_pos: usize,
    pub chs: Vec<char>,
    pub len: usize,
}

impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = self.line();
        write!(f, "{}", line)
    }
}

impl From<String> for GapBuffer {
    fn from(s: String) -> Self {
        let mut buffer = vec![];
        for ch in s.chars() {
            buffer.push(ch);
        }

        let len = buffer.len();

        Self { cur_pos: 0, chs: buffer, len }
    }
}

impl GapBuffer {
    pub fn update_len(&mut self) {
        self.len = self.chs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chs.is_empty()
    }

    pub fn new(chs: &str) -> Self {
        let mut buffer = vec![];
        for ch in chs.chars() {
            buffer.push(ch);
        }

        let len = buffer.len();

        todo!("`buffer` is empty, so the length is zero. Which messes up typing, and navigation.");
        Self { cur_pos: 0, chs: buffer, len }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.line().len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.line()[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            result.push_str(grapheme);
        }

        result
    }

    pub fn line(&self) -> String {
        let mut string = "".to_string();
        self.chs.iter().for_each(|x| {
            string = format!("{}{}", string, x);
        });

        string
    }

    pub fn display(&self) {
        let mut indexes = "".to_string();
        for i in 0..self.chs.len() {
            let nums = format!("{}", i);
            indexes.push_str(nums.as_str());
        }

        let mut output = "".to_string();
        for buffer in self.chs.clone() {
            output.push_str(buffer.clone().to_string().as_str());
        }

        println!("{}\n{}\n", indexes, output);
    }

    pub fn insert(&mut self, ch: char, x: usize) {
        if x >= self.chs.len() {
            self.chs.push(ch);
        } else if x == 0 {
            self.chs.insert(0, ch);
        } else {
            self.chs.insert(x, ch);
        }

        self.update_len();
    }

    pub fn insert_newline(&mut self) {
        self.chs = vec!['\n'];
    }

    pub fn enter(&mut self) {
    }

    pub fn delete(&mut self, x: usize) {
        let len = self.chs.len();
        if x >= len {
            self.chs.pop();
        } else {
            self.chs.remove(x);
        }

        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let start: Vec<char> = self.chs.iter().take(at).copied().collect();
        let end: Vec<char> = self.chs.iter().skip(at).copied().collect();
        let end: String = end.iter().collect();

        self.chs = start;
        self.update_len();
        Self::from(end)
    }
}

