use std::fmt;

#[derive(PartialEq)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Insert => "INSERT",
            Self::Normal => "NORMAL",
            Self::Command => "COMMAND",
        };

        write!(f, "MODE: {}", mode)
    }
}
