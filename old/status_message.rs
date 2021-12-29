use std::time::Instant;

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

impl From<&str> for StatusMessage {
    fn from(message: &str) -> Self {
        Self {
            time: Instant::now(),
            text: message.to_string(),
        }
    }
}
