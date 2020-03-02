use std::env;

const LEAF_PASSWORD_HASH: &str = "LEAF_PASSWORD_HASH";

pub struct Config {
    pub password_hash: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let password_hash = env::var(LEAF_PASSWORD_HASH)
            .map_err(|_| format!("{} is missing or invalid", LEAF_PASSWORD_HASH))?;

        Ok(Config { password_hash })
    }
}
