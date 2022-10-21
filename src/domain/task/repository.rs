use crate::domain::shared::repository::Repository;
use crate::domain::task::aggregate::Task;
use crate::domain::task::view::TaskView;

pub trait TaskRepository: Repository<TaskView, Task> {}
