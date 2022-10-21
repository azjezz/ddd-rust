use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskCommand {
    Create { content: String },
    Finish,
    Delete,
}

impl TaskCommand {
    pub fn create(content: String) -> Self {
        TaskCommand::Create { content }
    }
}
