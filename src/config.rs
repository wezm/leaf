use std::env;
use std::ffi::OsStr;

const LEAF_API_TOKEN: &str = "LEAF_API_TOKEN";
const LEAF_PASSWORD_HASH: &str = "LEAF_PASSWORD_HASH";
const LEAF_SECURE_COOKIE: &str = "LEAF_SECURE_COOKIE";
const MIN_TOKEN_LEN: usize = 64;

pub struct Config {
    pub password_hash: String,
    pub api_token: String,
    pub secure_cookie: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let password_hash = env::var(LEAF_PASSWORD_HASH)
            .map_err(|_| format!("{} is missing or invalid", LEAF_PASSWORD_HASH))?;
        let api_token = env::var(LEAF_API_TOKEN)
            .map_err(|_| format!("{} is missing or invalid", LEAF_API_TOKEN))?;
        if api_token.len() < MIN_TOKEN_LEN {
            return Err(format!(
                "{} is too short. At least {} chars required but got {} ",
                LEAF_API_TOKEN,
                MIN_TOKEN_LEN,
                api_token.len()
            ));
        }
        let secure_cookie = env::var_os(LEAF_SECURE_COOKIE)
            .map(|value| value != OsStr::new("false"))
            .unwrap_or(true);

        Ok(Config {
            password_hash,
            api_token,
            secure_cookie,
        })
    }
}
