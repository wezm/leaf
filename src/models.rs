use chrono::prelude::*;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::store::{self, AppendOnlyTaskList, ReadWriteTaskList};

// TODO: Move
pub type Store = Arc<Mutex<store::Store<ReadWriteTaskList, AppendOnlyTaskList>>>;

pub type TaskId = Ulid;
pub type Timestamp = DateTime<Utc>;

// TODO: Revisit visibility of structs and their fields
#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct CompletedTask<'task> {
    pub id: TaskId,
    pub description: &'task str,
    pub completed_at: Timestamp,
}

// Ideally we would use something like this for the form but serde_urlencoded
// as used by warp is severely limited when it comes to sequences. Not
// enough of the warp insides are public API to easily make a version of the form
// filter that uses serde_qs instead. Hopefully this improves in the future.
//#[derive(Debug, Deserialize)]
//pub struct TasksForm {
//    pub description: String,
//    pub completed: Vec<TaskId>
//}
