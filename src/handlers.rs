use super::templates;
use leaf::models::{NewTask, State, Task};
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

pub async fn create_task(task: NewTask, state: State) -> Result<Box<dyn warp::Reply>, Infallible> {
    log::debug!("create_task: {:?}", task);
    let mut store = state.lock().await;
    store.add(task); // TODO Handle result
    Ok(Box::new(redirect(Uri::from_static("/tasks"))))
}

pub async fn complete_task(id: u64, state: State) -> Result<impl warp::Reply, Infallible> {
    log::debug!("complete_task: id={}", id);

    let mut vec = state.lock().await;

    // TODO

    let deleted = false;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("    -> task id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}

pub fn redirect(uri: Uri) -> impl warp::Reply {
    let value =
        HeaderValue::from_maybe_shared(uri.to_string()).expect("Uri is a valid HeaderValue");
    warp::reply::with_header(StatusCode::FOUND, http::header::LOCATION, value)
}
