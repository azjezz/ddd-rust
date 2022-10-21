use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Task {
    pub content: String,
    pub is_finished: bool,
    pub is_deleted: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
