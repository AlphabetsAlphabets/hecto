use std::cmp;
use std::fs;

pub struct Row {
    pub string: String
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self { string: String::from(s) }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Unable to open file.");
        let mut rows = vec![];
        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Self { rows }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
