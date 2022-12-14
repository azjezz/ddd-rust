use crate::application::shared::quries::StandardQuery;
use crate::domain::task::aggregate::Task;
use crate::domain::task::view::TaskView;

pub type TaskQuery<R> = StandardQuery<R, TaskView, Task>;
