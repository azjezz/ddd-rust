use chrono::{DateTime, Utc};
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskEvent {
    Created {
        content: String,
        date: DateTime<Utc>,
    },
    Finished {
        date: DateTime<Utc>,
    },
    Deleted {
        date: DateTime<Utc>,
    },
}

impl DomainEvent for TaskEvent {
    fn event_type(&self) -> String {
        match self {
            TaskEvent::Created { .. } => "Created".to_string(),
            TaskEvent::Finished { .. } => "Finished".to_string(),
            TaskEvent::Deleted { .. } => "Deleted".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "0.1.0".to_string()
    }
}
