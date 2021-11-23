use std::fmt;

#[derive(PartialEq)]
pub enum Mode {
    Insert,
    Normal,
    Command,
    Save,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Insert => "INSERT",
            Self::Normal => "NORMAL",
            Self::Command => "COMMAND",
            Self::Save => "SAVE",
        };

        write!(f, "MODE: {}", mode)
    }
}
