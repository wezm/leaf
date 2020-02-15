use super::templates;
use leaf::models::{State, Task};
use std::convert::Infallible;
use warp::http::StatusCode;

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

pub async fn create_task(create: Task, state: State) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_task: {:?}", create);

    let mut vec = state.lock().await;

    // TODO

    Ok(StatusCode::CREATED)
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
