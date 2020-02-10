//! User authentication.

use warp::Filter;

/// The auth filters combined.
pub fn auth(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    new()
}

/// GET /login
pub fn new(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and_then(handlers::new)
}

mod handlers {
    use crate::templates;
    use std::convert::Infallible;

    pub async fn new() -> Result<impl warp::Reply, Infallible> {
        let page: templates::Layout<'_, _> = templates::Layout {
            title: "🍃 Login",
            body: templates::Login {},
        };
        Ok(warp::reply::html(page.to_string()))
    }
}
