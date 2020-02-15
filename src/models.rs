use chrono::prelude::*;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::store::{AppendOnlyTaskList, ReadWriteTaskList, Store};

// TODO: Move
pub type State = Arc<Mutex<Store<ReadWriteTaskList, AppendOnlyTaskList>>>;

pub type TaskId = Ulid;
pub type Timestamp = DateTime<Utc>;

// TODO: Revisit visibility of structs and their fields
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
