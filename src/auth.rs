//! User authentication.

use crate::{config, templates};
use std::sync::Arc;

use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, LenientForm, Request};
use rocket::response::{content, Flash, Redirect};
use rocket::{Route, State};

pub type Config = Arc<config::Config>;

#[derive(FromForm)]
struct Login {
    password: String,
}

#[derive(Debug)]
pub struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
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
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        // Ok(Redirect::to(uri!(index)))
        Ok(Redirect::to("/")) // FIXME
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid password.",
        ))
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
fn login_user(_user: User) -> Redirect {
    //Redirect::to(uri!(index))
    Redirect::to("/") // FIXME
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> content::Html<String> {
    let page: templates::Layout<'_, _> = templates::Layout {
        title: "🍃 Login",
        body: templates::Login {
            flash: flash.as_ref().map(|flash| flash.msg()),
        },
    };
    content::Html(page.to_string())
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
