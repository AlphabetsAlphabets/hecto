use std::cmp;

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
    pub fn open() -> Self {
        let mut rows = vec![];
        rows.push(Row::from("Hello, world!"));
        Self { rows }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}
