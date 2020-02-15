use super::handlers;
use leaf::models::{State, Task};
use warp::Filter;

/// The 4 TODOs filters combined.
pub fn tasks(
    state: State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_list(state.clone())
        .or(tasks_create(state.clone()))
        .or(tasks_complete(state))
}

/// GET /tasks
pub fn tasks_list(
    state: State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::list_tasks)
}

/// POST /tasks with JSON body
pub fn tasks_create(
    state: State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::post())
        .and(json_body())
        .and(with_state(state))
        .and_then(handlers::create_task)
}

/// DELETE /tasks/:id
pub fn tasks_complete(
    state: State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("tasks" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /tasks/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_state(state))
        .and_then(handlers::complete_task)
}

fn with_state(
    state: State,
) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn json_body() -> impl Filter<Extract = (Task,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
