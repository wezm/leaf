//! User authentication.

use handlers::Credentials;
use warp::{reject, Filter};

use crate::{config, filters};
use std::collections::HashSet;
use std::sync::Arc;
use time::Duration;
use tokio::sync::Mutex;

pub const COOKIE_MAX_AGE: Duration = Duration::weeks(1);
pub const LEAF_SESSION: &str = "LEAF_SESSION";

/// Session store for active sessions.
///
/// https://owasp.org/www-project-cheat-sheets/cheatsheets/Session_Management_Cheat_Sheet
pub type SessionStore = Arc<Mutex<HashSet<String>>>;
pub type Config = Arc<config::Config>;

/// The auth filters combined.
pub fn auth(
    config: Config,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    new(Arc::clone(&config), Arc::clone(&sessions)).or(login(config, sessions))
}

/// GET /login
pub fn new(
    config: Config,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and(with_config(config))
        .and(with_sessions(sessions))
        .and_then(|config, sessions| handlers::new(config, sessions, None))
}

/// POST /login
pub fn login(
    config: Config,
    sessions: SessionStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(with_config(config))
        .and(with_sessions(sessions))
        .and(filters::form_body())
        .and_then(handlers::login)
}

//pub fn login_required() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
//
//}

fn with_sessions(
    store: SessionStore,
) -> impl Filter<Extract = (SessionStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&store))
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&config))
}

mod handlers {
    use crate::handlers::redirect;
    use serde::Deserialize;
    use std::convert::Infallible;
    use std::env;
    use warp::http::{Response, StatusCode, Uri};

    use super::LEAF_SESSION;
    use super::{Config, SessionStore};
    use crate::auth::COOKIE_MAX_AGE;
    use crate::templates;
    use cookie::{Cookie, CookieBuilder};
    use std::time::Duration;

    type SessionId = [u8; 8];

    #[derive(Debug, Deserialize, Clone)]
    pub struct Credentials {
        password: String,
    }

    pub async fn new(
        config: Config,
        sessions: SessionStore,
        message: Option<String>,
    ) -> Result<impl warp::Reply, Infallible> {
        let page: templates::Layout<'_, _> = templates::Layout {
            title: "🍃 Login",
            body: templates::Login { flash: message },
        };
        Ok(warp::reply::html(page.to_string()))
    }

    pub async fn login(
        config: Config,
        sessions: SessionStore,
        credentials: Credentials,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        if verify(&config.password_hash, credentials.password.as_bytes()) {
            let mut sessions = sessions.lock().await;

            let session_id = generate_session_id().expect("getrandom fail"); // FIXME
            let cookie = Cookie::build(LEAF_SESSION, &session_id)
                .path("/")
                .secure(config.secure_cookie)
                .http_only(true)
                .max_age(COOKIE_MAX_AGE)
                .finish();

            // FIXME: Location should be absolute URL?
            let reply = warp::reply::with_header(
                redirect(Uri::from_static("/")),
                "set-cookie",
                cookie.to_string(),
            );

            // Add to sessions
            sessions.insert(session_id);

            Ok(Box::new(reply))
        } else {
            let page = new(config, sessions, Some(String::from("Invalid password.")))
                .await
                .unwrap();
            Ok(Box::new(page))
        }
    }

    fn verify(hash: &str, password: &[u8]) -> bool {
        argon2::verify_encoded(hash, password).unwrap_or(false)
    }

    // TODO Add a logout path that deletes from the session store and cookie
    fn generate_session_id() -> Result<String, getrandom::Error> {
        // TODO: spawn an async task
        let mut session_id = [0; 8];
        getrandom::getrandom(&mut session_id)?;
        Ok(encode_session_id(&session_id))
    }

    fn encode_session_id(session_id: &SessionId) -> String {
        base64::encode_config(session_id, base64::URL_SAFE_NO_PAD)
    }

    fn decode_session_id(session_id: &str) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode_config(session_id, base64::URL_SAFE_NO_PAD)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_encode_session_id() {
            let session_id = [255; 8];
            assert_eq!(&encode_session_id(&session_id), "__________8");
            let session_id = [1, 2, 3, 4, 5, 6, 7, 8];
            assert_eq!(&encode_session_id(&session_id), "AQIDBAUGBwg");
        }

        #[test]
        fn test_decode_session_id() {
            let session_id = "__________8";
            assert_eq!(&decode_session_id(session_id).unwrap(), &[255; 8]);
            let session_id = "AQIDBAUGBwg";
            assert_eq!(
                &decode_session_id(&session_id).unwrap(),
                &[1, 2, 3, 4, 5, 6, 7, 8]
            );
        }
    }
}

#[derive(Debug)]
pub struct NotAuthorised;

impl reject::Reject for NotAuthorised {}
