use std::fmt;
use std::cmp;

use unicode_segmentation::UnicodeSegmentation;
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

const NULL_BYTE: char = b'\0' as char;

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
    pub fn new(chs: &str) -> Self {
        let mut buffer = vec![];
        for ch in chs.chars() {
            buffer.push(ch);
        }

        let len = buffer.len();

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

    fn left(&mut self) {
        self.cur_pos -= 1;
    }

    fn right(&mut self, ch: char) {
        self.cur_pos += 1;
    }

    fn grow(&mut self, ch: char) {
        self.chs.push('_');
        self.insert(ch);
    }

    pub fn insert(&mut self, character: char) {
        if let Some(ch) = self.chs.get_mut(self.cur_pos) {
            *ch = character;
            let right = ch.clone();
            self.right(right);
        } else {
            self.grow(character);
        }
    }
}

