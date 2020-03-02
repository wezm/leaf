//! Static files.

use rocket::response::content;
use rocket::Route;

const CSS: &str = include_str!("app.css");

pub fn routes() -> Vec<Route> {
    routes![css]
}

#[get("/app.css")]
pub fn css() -> content::Css<&'static str> {
    content::Css(CSS)
}
