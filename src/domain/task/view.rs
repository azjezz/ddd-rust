use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// The view for a TaskView query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskView {
    pub id: Option<String>,
    pub content: String,
    pub is_finished: bool,
    pub is_deleted: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
