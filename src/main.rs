#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod auth;
mod config;
mod form;
mod public;
mod templates;
#[cfg(test)]
mod tests;

use std::env;
use std::ffi::OsString;
use std::process::exit;
use std::sync::{Arc, Mutex};

use rocket::request::{FlashMessage, LenientForm};
use rocket::response::{content, Flash, Redirect};
use rocket::{Rocket, State};

use leaf::models::{NewTask, Store};
use leaf::store::{self, AppendOnlyTaskList, ReadWriteTaskList};

use config::Config;
use form::TasksForm;

use auth::User;

const LOG_ENV_VAR: &str = "LEAF_LOG";
const LEAF_TASKS_PATH: &str = "LEAF_TASKS_PATH";
const LEAF_COMPLETED_PATH: &str = "LEAF_COMPLETED_PATH";

#[get("/")]
fn index(_user: User, msg: Option<FlashMessage>, state: State<Store>) -> content::Html<String> {
    let store = state.lock().unwrap();
    let page: templates::Layout<'_, _> = templates::Layout {
        title: "🍃 Tasks",
        body: templates::Index {
            tasks: store.list(),
        },
    };
    content::Html(page.to_string())
}

#[get("/", rank = 2)]
fn index_logged_out() -> Redirect {
    Redirect::to(uri!(auth::login_page))
}

#[post("/", data = "<form>")]
fn form(
    _user: User,
    form: LenientForm<TasksForm>,
    state: State<Store>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();
    let mut store = state.lock().unwrap();

    // Create new task if present
    if let Some(description) = form.new_task {
        let task = NewTask { description };
        log::debug!("create_task: {:?}", task);
        store
            .add(task)
            .map_err(|_err| Flash::error(Redirect::to("/"), "Failed to add new task"))?;
    }

    // Complete any checked tasks
    store
        .complete(&form.completed_ids)
        .map_err(|_err| Flash::error(Redirect::to("/"), "Failed to complete tasks"))?;

    Ok(Redirect::to("/"))
}

#[get("/app.css")]
fn css() -> content::Css<&'static str> {
    content::Css(public::CSS)
}

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
        .mount("/", routes![index, index_logged_out, css])
        .mount("/tasks", routes![form])
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
