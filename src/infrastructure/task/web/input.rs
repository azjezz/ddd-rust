use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateTask {
    pub content: String,
}
