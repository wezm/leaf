//! Task handling routes.

use rocket::request::{FlashMessage, LenientForm};
use rocket::response::{content, Flash, Redirect};
use rocket::{Route, State};

use leaf::models::{NewTask, Store};

use crate::auth::{self, User, UserOrToken};
use crate::form::TasksForm;
use crate::templates;

pub fn routes() -> Vec<Route> {
    routes![index, index_logged_out, form]
}

#[get("/")]
fn index(user: User, _msg: Option<FlashMessage>, state: State<Store>) -> content::Html<String> {
    let store = state.lock().unwrap();
    let page: templates::Layout<'_, '_, _> = templates::Layout {
        title: "Tasks",
        body: templates::Index {
            tasks: store.list(),
        },
        user: Some(&user),
    };
    content::Html(page.to_string())
}

#[get("/", rank = 2)]
fn index_logged_out() -> Redirect {
    Redirect::to(uri!(auth::login_page))
}

#[post("/tasks", data = "<form>")]
fn form(
    _auth: UserOrToken,
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
