use std::env;
use std::ffi::OsStr;

const LEAF_PASSWORD_HASH: &str = "LEAF_PASSWORD_HASH";
const LEAF_SECURE_COOKIE: &str = "LEAF_SECURE_COOKIE";

pub struct Config {
    pub password_hash: String,
    pub secure_cookie: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let password_hash = env::var(LEAF_PASSWORD_HASH)
            .map_err(|_| format!("{} is missing or invalid", LEAF_PASSWORD_HASH))?;

        let secure_cookie = env::var_os(LEAF_SECURE_COOKIE)
            .map(|value| value != OsStr::new("false"))
            .unwrap_or(true);

        Ok(Config {
            password_hash,
            secure_cookie,
        })
    }
}
