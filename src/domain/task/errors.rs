use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct TaskError(String);

impl From<&str> for TaskError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TaskError {}
