use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use chrono::prelude::*;
use rusty_ulid::Ulid;

use crate::models::*; // FIXME: *

// This module operates under the assumption that the active task list will generally remain
// fairly small, but the completed list will be more or less ever growing. This, we typically
// write out the whole active task list to a new file and move it into place but append only
// the the completed list.

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Csv(csv::Error),
}

pub trait CreateTask {
    fn create(&mut self, task: NewTask) -> Result<TaskId, Error>;
}

pub trait AddTasks {
    fn add(&mut self, tasks: &[&Task]) -> Result<(), Error>;
}

pub trait ListTasks {
    fn list(&self) -> &[Task];
}

pub trait RemoveTasks {
    fn remove(
        &mut self,
        task_ids: &[TaskId],
        body: impl FnMut(Vec<&Task>) -> Result<(), Error>,
    ) -> Result<(), Error>;
}

pub struct Store<Tasks, Completed>
where
    Tasks: CreateTask + RemoveTasks + ListTasks,
    Completed: AddTasks,
{
    tasks: Tasks,
    completed: Completed,
}

pub struct ReadWriteTaskList {
    tasks: Vec<Task>,
    path: PathBuf,
}

pub struct AppendOnlyTaskList {
    writer: csv::Writer<File>,
}

impl<Tasks, Completed> Store<Tasks, Completed>
where
    Tasks: CreateTask + RemoveTasks + ListTasks,
    Completed: AddTasks,
{
    pub fn new(tasks: Tasks, completed: Completed) -> Self {
        Store { tasks, completed }
    }

    pub fn add(&mut self, task: NewTask) -> Result<TaskId, Error> {
        self.tasks.create(task)
    }

    pub fn complete(&mut self, task_ids: &[TaskId]) -> Result<(), Error> {
        let completed = &mut self.completed;
        self.tasks
            .remove(task_ids, |removed_tasks| completed.add(&removed_tasks))
    }

    pub fn list(&self) -> &[Task] {
        self.tasks.list()
    }
}

impl ReadWriteTaskList {
    pub fn new<P: AsRef<OsStr>>(path: P) -> Result<Self, Error> {
        // Attempt to read the records in from the file to populate the vec of tasks
        let path = Path::new(&path).to_owned();
        let tasks = Self::read_tasks(&path)?;

        Ok(ReadWriteTaskList { tasks, path })
    }

    fn read_tasks(path: &Path) -> Result<Vec<Task>, Error> {
        match File::open(path) {
            Ok(file) => {
                let file = BufReader::new(file);
                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(file);
                rdr.deserialize()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(Error::from)
            }
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(Vec::new()),
                _ => return Err(Error::from(err)),
            },
        }
    }

    fn write_tasks(tasks: &[&Task], file: &mut File) -> Result<(), Error> {
        let mut builder = csv::WriterBuilder::new();
        let mut writer = builder.has_headers(false).from_writer(file);
        for &task in tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;
        Ok(())
    }
}

impl CreateTask for ReadWriteTaskList {
    fn create(&mut self, new_task: NewTask) -> Result<TaskId, Error> {
        // Add the new task to self, then append it to the file

        let id = Ulid::generate();
        let task = Task {
            id,
            description: new_task.description,
        };

        // Append new item to file
        let mut options = OpenOptions::new();
        let file = options.create(true).append(true).open(&self.path)?;
        let mut builder = csv::WriterBuilder::new();
        let mut writer = builder.has_headers(false).from_writer(file);
        writer.serialize(&task)?;
        writer.flush()?;

        self.tasks.push(task);

        Ok(id)
    }
}

impl RemoveTasks for ReadWriteTaskList {
    fn remove(
        &mut self,
        task_ids: &[TaskId],
        mut body: impl FnMut(Vec<&Task>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        // Open a temp file in the same directory as the target file
        let temp_path = self.path.with_extension("tmp");

        // Block to scope file
        let keep = {
            let mut file = File::create(&temp_path)?;

            // Collect new list of tasks
            let (remove, keep): (Vec<&Task>, Vec<&Task>) = self
                .tasks
                .iter()
                .partition(|task| task_ids.contains(&task.id));

            // Write out all tasks
            Self::write_tasks(&keep, &mut file)?;

            // Call the body
            body(remove)?;
            keep
        };

        // Move into place if body was successful
        fs::rename(temp_path, &self.path)?;

        // Update ourselves with the new task list
        self.tasks = keep.into_iter().cloned().collect();

        Ok(())
    }
}

impl ListTasks for ReadWriteTaskList {
    fn list(&self) -> &[Task] {
        self.tasks.as_slice()
    }
}

impl AppendOnlyTaskList {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Attempt to open the file for appending
        let mut options = OpenOptions::new();
        let file = options.create(true).append(true).open(path)?;
        let mut builder = csv::WriterBuilder::new();
        let writer = builder.has_headers(false).from_writer(file);

        Ok(AppendOnlyTaskList { writer })
    }
}

impl AddTasks for AppendOnlyTaskList {
    fn add(&mut self, tasks: &[&Task]) -> Result<(), Error> {
        for &task in tasks {
            let completed_task = CompletedTask::from(task);
            self.writer.serialize(completed_task)?;
        }

        self.writer.flush()?;
        Ok(())
    }
}

impl NewTask {
    pub fn new(description: String) -> Self {
        NewTask { description }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Error::Csv(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Csv(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl<'task> From<&'task Task> for CompletedTask<'task> {
    fn from(task: &'task Task) -> Self {
        CompletedTask {
            id: task.id,
            description: &task.description,
            completed_at: Utc::now().trunc_subsecs(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TASKS_FILENAME: &str = "tasks.csv";
    const COMPLETED_FILENAME: &str = "completed.csv";

    #[test]
    fn test() {
        // TODO: Write tests for more scenarios
        let testdir = tempfile::tempdir().expect("unable to create tempdir");
        let (id1, id2, tasks_path, completed_path) = {
            let tasks_path = testdir.path().join(TASKS_FILENAME);
            let completed_path = testdir.path().join(COMPLETED_FILENAME);

            let tasks = ReadWriteTaskList::new(&tasks_path).expect(TASKS_FILENAME);
            let completed = AppendOnlyTaskList::new(&completed_path).expect(COMPLETED_FILENAME);
            let mut store = Store::new(tasks, completed);

            let task1 = NewTask::new(String::from("do a thing"));
            let task2 = NewTask::new(String::from("do another thing"));
            let id1 = store.add(task1).unwrap();
            let id2 = store.add(task2).unwrap();
            store.complete(&[id1]).expect("complete");
            (id1, id2, tasks_path, completed_path)
        };

        // Now check on the state of the files
        let tasks_csv = fs::read_to_string(tasks_path).unwrap();
        let completed_csv = fs::read_to_string(completed_path).unwrap();
        assert_eq!(
            format!("id,description\n{},do another thing\n", id2),
            tasks_csv
        );
        // TODO: test completed_at...
        assert!(completed_csv.starts_with(&format!("{},do a thing,", id1)));
    }
}
