//! Static files.

use warp::Filter;

/// The static files combined.
pub fn files() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    css()
}

fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("app.css")
        .and(warp::get())
        .and_then(handlers::css)
}

mod handlers {
    use std::convert::Infallible;
    use warp::http::Response;

    const CSS: &str = include_str!("app.css");

    pub async fn css() -> Result<impl warp::Reply, Infallible> {
            Ok(Response::builder()
                .header("Content-Type", "text/css")
                .body(CSS))
    }
}
