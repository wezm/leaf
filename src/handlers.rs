use super::templates;
use leaf::models::{NewTask, State, Task, TaskId, TasksForm};
use rusty_ulid::Ulid;
use std::convert::Infallible;
use warp::http::header::HeaderValue;
use warp::http::{self, StatusCode, Uri};

pub async fn list_tasks(state: State) -> Result<impl warp::Reply, Infallible> {
    let store = state.lock().await;
    let page: templates::Layout<'_, _> = templates::Layout {
        title: "🍃 Tasks",
        body: templates::Index {
            tasks: store.list(),
        },
    };
    Ok(warp::reply::html(page.to_string()))
}

pub async fn handle_tasks_form(
    form: TasksForm,
    state: State,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let mut store = state.lock().await;

    let description = form
        .get("description")
        .map(|value| value.trim())
        .unwrap_or_default();

    // Create new task if present
    if !description.is_empty() {
        let task = NewTask {
            description: description.to_string(),
        };
        log::debug!("create_task: {:?}", task);
        store.add(task).expect("failed to add"); // TODO Handle result
    }

    // Complete any checked tasks
    let completed_ids = form
        .iter()
        .filter_map(|(key, value)| {
            if key.starts_with("complete") {
                value.parse().ok()
            } else {
                None
            }
        })
        .collect::<Vec<TaskId>>();
    store.complete(&completed_ids).expect("failed to complete"); // FIXME

    Ok(Box::new(redirect(Uri::from_static("/"))))
}

pub fn redirect(uri: Uri) -> impl warp::Reply {
    let value =
        HeaderValue::from_maybe_shared(uri.to_string()).expect("Uri is a valid HeaderValue");
    warp::reply::with_header(StatusCode::FOUND, http::header::LOCATION, value)
}
