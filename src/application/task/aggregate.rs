use crate::application::task::services::TaskServices;
use crate::domain::task::aggregate::Task;
use crate::domain::task::commands::TaskCommand;
use crate::domain::task::errors::TaskError;
use crate::domain::task::events::TaskEvent;

use async_trait::async_trait;
use chrono::Utc;
use cqrs_es::Aggregate;

#[async_trait]
impl Aggregate for Task {
    type Command = TaskCommand;
    type Event = TaskEvent;
    type Error = TaskError;
    type Services = TaskServices;

    fn aggregate_type() -> String {
        "task".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let event = match command {
            TaskCommand::Create { content } => TaskEvent::Created {
                content,
                date: Utc::now(),
            },
            TaskCommand::Finish { .. } => TaskEvent::Finished { date: Utc::now() },
            TaskCommand::Delete { .. } => TaskEvent::Deleted { date: Utc::now() },
        };

        Ok(vec![event])
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TaskEvent::Created { content, date } => {
                self.content = content;
                self.created_at = Some(date);
            }
            TaskEvent::Finished { date } => {
                self.is_finished = true;
                self.finished_at = Some(date);
            }
            TaskEvent::Deleted { date } => {
                self.is_deleted = true;
                self.deleted_at = Some(date);
            }
        }
    }
}
