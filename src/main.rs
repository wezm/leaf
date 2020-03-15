#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod auth;
mod config;
mod form;
mod public;
mod tasks;
mod templates;

use std::error::Error as StdError;
use std::ffi::OsString;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, fmt};

use rocket::Rocket;

use config::Config;
use leaf::store::{self, AppendOnlyTaskList, ReadWriteTaskList};

const LEAF_TASKS_PATH: &str = "LEAF_TASKS_PATH";
const LEAF_COMPLETED_PATH: &str = "LEAF_COMPLETED_PATH";

#[derive(Debug)]
struct StoreError {
    path: OsString,
    source: leaf::store::Error,
}

fn rocket() -> Result<Rocket, StoreError> {
    let tasks_path = env::var_os(LEAF_TASKS_PATH).unwrap_or_else(|| OsString::from("tasks.csv"));
    let completed_path =
        env::var_os(LEAF_COMPLETED_PATH).unwrap_or_else(|| OsString::from("completed.csv"));
    let tasks = ReadWriteTaskList::new(&tasks_path).map_err(|err| StoreError {
        path: tasks_path,
        source: err,
    })?;
    let completed = AppendOnlyTaskList::new(&completed_path).map_err(|err| StoreError {
        path: completed_path,
        source: err,
    })?;
    let store = store::Store::new(tasks, completed);
    let store = Arc::new(Mutex::new(store));

    let config = Config::from_env().unwrap_or_else(exit_config_error);
    let config = Arc::new(config);

    let server = rocket::ignite()
        .mount("/", auth::routes())
        .mount("/", tasks::routes())
        .mount("/", public::routes())
        .manage(config)
        .manage(store);

    Ok(server)
}

fn main() {
    let rocket = match rocket() {
        Ok(rocket) => rocket,
        Err(err) => {
            eprintln!("{}", err);
            if let Some(source) = err.source() {
                eprintln!(" - Caused by: {}", source);
            }
            exit(1);
        }
    };
    rocket.launch();
}

fn exit_config_error(err: String) -> Config {
    eprintln!(
        "Configuration error:\n\n{}\n\nSee https://github.com/wezm/leaf-tasks#configuration",
        err
    );
    exit(2);
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to initialise store ({})",
            self.path.to_string_lossy()
        )
    }
}

impl StdError for StoreError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
