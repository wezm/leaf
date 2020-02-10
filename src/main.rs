use std::env;
use warp::Filter;

const LOG_ENV_VAR: &str = "LEAF_LOG";

mod auth;
//mod filters;
//mod handlers;
mod public;
mod store;
mod templates;

#[tokio::main]
async fn main() {
    if env::var_os(LOG_ENV_VAR).is_none() {
        // Set `LEAF_LOG=leaf=debug` to see debug logs, this only shows access logs.
        env::set_var(LOG_ENV_VAR, "leaf=info");
    }
    let env = env_logger::Env::new().filter(LOG_ENV_VAR);
    env_logger::init_from_env(env);

    let api = auth::auth().or(public::files());

    // View access logs by setting `LEAF_LOG=leaf`.
    let routes = api.with(warp::log("leaf"));
    // Start up the server...
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
