//! User authentication.

use std::sync::Arc;

use hyper::header::Header;
use rocket::http::hyper::header::{Authorization, Bearer};
use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, LenientForm, Request};
use rocket::response::{content, Flash, Redirect};
use rocket::{Route, State};
use time::Duration;

use crate::{config, tasks, templates};

pub const LEAF_SESSION: &str = "LEAF_SESSION";

pub type Config = Arc<config::Config>;

#[derive(Debug)]
pub enum TokenError {
    Invalid,
}

pub struct User(usize);
pub struct Token(String);
pub enum UserOrToken {
    User(User),
    Token(Token),
}

#[derive(FromForm)]
struct Login {
    password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private(LEAF_SESSION)
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token, Self::Error> {
        use request::Outcome;

        request
            .headers()
            .get_one("Authorization")
            .and_then(|value| Header::parse_header(&[value.as_bytes().to_vec()]).ok())
            .and_then(|token: Authorization<Bearer>| {
                let config = request.guard::<State<Config>>().unwrap(); // NOTE(unwrap): Config should always be available
                if token.0.token == config.api_token {
                    Some(Outcome::Success(Token(token.0.token)))
                } else {
                    Some(Outcome::Failure((
                        Status::Unauthorized,
                        TokenError::Invalid,
                    )))
                }
            })
            .unwrap_or_else(|| Outcome::Forward(()))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserOrToken {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserOrToken, Self::Error> {
        use request::Outcome;

        match request.guard::<User>().map(UserOrToken::User) {
            Outcome::Success(user_or_token) => Outcome::Success(user_or_token),
            _ => request.guard::<Token>().map(UserOrToken::Token),
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![login, logout, login_user, login_page]
}

#[post("/login", data = "<login>")]
fn login(
    mut cookies: Cookies,
    login: LenientForm<Login>,
    config: State<Config>,
) -> Result<Redirect, Flash<Redirect>> {
    if verify(&config.password_hash, login.password.as_bytes()) {
        let cookie = Cookie::build(LEAF_SESSION, 1.to_string())
            .path("/")
            .secure(config.secure_cookie)
            .http_only(true)
            .max_age(Duration::weeks(1))
            .finish();

        cookies.add_private(cookie);
        Ok(Redirect::to(uri!(tasks::index)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid password.",
        ))
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(LEAF_SESSION));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(tasks::index))
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> content::Html<String> {
    let page: templates::Layout<'_, '_, _> = templates::Layout {
        title: "🍃 Login",
        body: templates::Login {
            flash: flash.as_ref().map(|flash| flash.msg()),
        },
        user: None,
    };
    content::Html(page.to_string())
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
