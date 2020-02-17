use super::handlers;
use crate::auth;
use crate::auth::SessionStore;
use leaf::models::{NewTask, State, Task, TaskId, TasksForm};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use warp::Filter;

/// The 4 TODOs filters combined.
pub fn tasks(
    state: State,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_list(Arc::clone(&state), Arc::clone(&sessions)).or(tasks_form(state, sessions))
}

/// GET /tasks
pub fn tasks_list(
    state: State,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(auth::login_required(sessions))
        .untuple_one()
        .and(with_state(state))
        .and_then(handlers::list_tasks)
}

/// POST /tasks with form body
///
/// Creates new tasks and completes existing ones.
pub fn tasks_form(
    state: State,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::post())
        .and(auth::login_required(sessions))
        .untuple_one()
        .and(form_body())
        .and(with_state(state))
        .and_then(handlers::handle_tasks_form)
}

///// DELETE /tasks/:id
//pub fn tasks_complete(
//    state: State,
//) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//    // We'll make one of our endpoints admin-only to show how authentication filters are used
//    let admin_only = warp::header::exact("authorization", "Bearer admin");
//
//    warp::path!("tasks" / u64)
//        // It is important to put the auth check _after_ the path filters.
//        // If we put the auth check before, the request `PUT /tasks/invalid-string`
//        // would try this filter and reject because the authorization header doesn't match,
//        // rather because the param is wrong for that other path.
//        .and(admin_only)
//        .and(warp::delete())
//        .and(with_state(state))
//        .and_then(handlers::complete_task)
//}

fn with_state(
    state: State,
) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn form_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a form body
    // (and to reject large payloads)...
    warp::body::content_length_limit(16 * 1024).and(warp::body::form::<T>())
}
