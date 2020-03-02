#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod auth;
mod config;
mod form;
mod public;
mod tasks;
mod templates;

use std::env;
use std::ffi::OsString;
use std::process::exit;
use std::sync::{Arc, Mutex};

use rocket::Rocket;

use leaf::store::{self, AppendOnlyTaskList, ReadWriteTaskList};
use config::Config;

const LOG_ENV_VAR: &str = "LEAF_LOG";
const LEAF_TASKS_PATH: &str = "LEAF_TASKS_PATH";
const LEAF_COMPLETED_PATH: &str = "LEAF_COMPLETED_PATH";

fn rocket() -> Rocket {
    //    let env = env_logger::Env::new().filter(LOG_ENV_VAR);
    //    env_logger::init_from_env(env);

    let tasks_path = env::var_os(LEAF_TASKS_PATH).unwrap_or_else(|| OsString::from("tasks.csv"));
    let completed_path =
        env::var_os(LEAF_COMPLETED_PATH).unwrap_or_else(|| OsString::from("completed.csv"));
    let tasks = ReadWriteTaskList::new(&tasks_path).expect("FIXME tasks.csv");
    let completed = AppendOnlyTaskList::new(&completed_path).expect("FIXME");
    let store = store::Store::new(tasks, completed);
    let store = Arc::new(Mutex::new(store));

    let config = Config::from_env().unwrap_or_else(exit_config_error);
    let config = Arc::new(config);

    rocket::ignite()
        .mount("/", auth::routes())
        .mount("/", tasks::routes())
        .mount("/", public::routes())
        .manage(config)
        .manage(store)
}

fn main() {
    rocket().launch();
}

fn exit_config_error(err: String) -> Config {
    eprintln!(
        "Configuration error:\n\n{}\n\nSee https://github.com/wezm/leaf-tasks#configuration",
        err
    );
    exit(1);
}
