//! User authentication.

use handlers::Credentials;
use warp::Filter;

pub const LEAF_PASSWORD_HASH: &str = "LEAF_PASSWORD_HASH";

/// The auth filters combined.
pub fn auth() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    new().or(login())
}

/// GET /login
pub fn new() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and_then(|| handlers::new(None))
}

/// POST /login
pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::form::<Credentials>())
        .and_then(handlers::login)
}

mod handlers {
    use crate::handlers::redirect;
    use serde::Deserialize;
    use std::convert::Infallible;
    use std::env;
    use warp::http::{Response, StatusCode, Uri};

    use super::LEAF_PASSWORD_HASH;
    use crate::templates;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Credentials {
        password: String,
    }

    pub async fn new(message: Option<String>) -> Result<impl warp::Reply, Infallible> {
        let page: templates::Layout<'_, _> = templates::Layout {
            title: "🍃 Login",
            body: templates::Login { flash: message },
        };
        Ok(warp::reply::html(page.to_string()))
    }

    pub async fn login(credentials: Credentials) -> Result<Box<dyn warp::Reply>, Infallible> {
        // TODO: Pass this down via state
        // NOTE(unwrap): Safe as main checks that it's set
        let hash = env::var(LEAF_PASSWORD_HASH).unwrap();
        if verify(&hash, credentials.password.as_bytes()) {
            // FIXME: Location should be absolute URL?
            Ok(Box::new(redirect(Uri::from_static("/"))))
        } else {
            let page = new(Some(String::from("Invalid password."))).await.unwrap();
            Ok(Box::new(page))
        }
    }

    fn verify(hash: &str, password: &[u8]) -> bool {
        argon2::verify_encoded(hash, password).unwrap_or(false)
    }
}
