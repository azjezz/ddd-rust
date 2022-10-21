use crate::domain::task::aggregate::Task;
use crate::domain::task::events::TaskEvent;
use crate::domain::task::view::TaskView;

use cqrs_es::EventEnvelope;
use cqrs_es::View;

// This updates the view with events as they are committed.
// The logic should be minimal here
impl View<Task> for TaskView {
    fn update(&mut self, event: &EventEnvelope<Task>) {
        match &event.payload {
            TaskEvent::Created { content, date } => {
                self.id = Some(event.aggregate_id.to_string());
                self.content = content.clone();
                self.created_at = Some(*date);
            }
            TaskEvent::Finished { date } => {
                self.is_finished = true;
                self.finished_at = Some(*date);
            }
            TaskEvent::Deleted { date } => {
                self.is_deleted = true;
                self.deleted_at = Some(*date);
            }
        }
    }
}
