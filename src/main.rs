mod auth;
mod config;
mod filters;
mod handlers;
mod public;
mod templates;

use std::env;
use std::ffi::OsString;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

use leaf::store::{AppendOnlyTaskList, ReadWriteTaskList, Store};
use std::collections::HashSet;

const LOG_ENV_VAR: &str = "LEAF_LOG";
const LEAF_TASKS_PATH: &str = "LEAF_TASKS_PATH";
const LEAF_COMPLETED_PATH: &str = "LEAF_COMPLETED_PATH";

use config::Config;

#[tokio::main]
async fn main() {
    if env::var_os(LOG_ENV_VAR).is_none() {
        // Set `LEAF_LOG=leaf=debug` to see debug logs, this only shows access logs.
        env::set_var(LOG_ENV_VAR, "leaf=info");
    }
    let env = env_logger::Env::new().filter(LOG_ENV_VAR);
    env_logger::init_from_env(env);

    let config = Config::from_env().unwrap_or_else(exit_config_error);

    let tasks_path = env::var_os(LEAF_TASKS_PATH).unwrap_or_else(|| OsString::from("tasks.csv"));
    let completed_path =
        env::var_os(LEAF_COMPLETED_PATH).unwrap_or_else(|| OsString::from("completed.csv"));
    let tasks = ReadWriteTaskList::new(&tasks_path).expect("FIXME tasks.csv");
    let completed = AppendOnlyTaskList::new(&completed_path).expect("FIXME");
    let store = Store::new(tasks, completed);
    let state = Arc::new(Mutex::new(store));

    let config = Arc::new(config);
    let session_store = Arc::new(Mutex::new(HashSet::new()));

    let api = auth::auth(config, session_store)
        .or(filters::tasks(state))
        .or(public::files());

    // View access logs by setting `LEAF_LOG=leaf`.
    let routes = api.with(warp::log("leaf"));
    // Start up the server...
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

fn exit_config_error(err: String) -> Config {
    eprintln!(
        "Configuration error:\n\n{}\n\nSee https://github.com/wezm/leaf-tasks#configuration",
        err
    );
    exit(1);
}
