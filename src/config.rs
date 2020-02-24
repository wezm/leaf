use std::env;
use std::ffi::OsStr;

const LEAF_PASSWORD_HASH: &str = "LEAF_PASSWORD_HASH";
const LEAF_SECURE_COOKIE: &str = "LEAF_SECURE_COOKIE";
const LEAF_SECRET_KEY_BASE: &str = "LEAF_SECRET_KEY_BASE";

const SECRET_KEY_BASE_BYTES: usize = 32;

pub struct Config {
    pub password_hash: String,
    pub secure_cookie: bool,
    pub secret_key_base: [u8; SECRET_KEY_BASE_BYTES],
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let password_hash = env::var(LEAF_PASSWORD_HASH)
            .map_err(|_| format!("{} is missing or invalid", LEAF_PASSWORD_HASH))?;

        let secure_cookie = env::var_os(LEAF_SECURE_COOKIE)
            .map(|value| value != OsStr::new("false"))
            .unwrap_or(true);

        let secret_key_base = env::var(LEAF_SECRET_KEY_BASE)
            .map_err(|_| format!("{} is missing or invalid", LEAF_SECRET_KEY_BASE))?;
        let secret_key_base = parse_secret_key_base(&secret_key_base)?;

        Ok(Config {
            password_hash,
            secure_cookie,
            secret_key_base,
        })
    }
}

fn parse_secret_key_base(base: &str) -> Result<[u8; SECRET_KEY_BASE_BYTES], String> {
    base.chars()
        .map(from_hex)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|digits| {
            if digits.len() != SECRET_KEY_BASE_BYTES * 2 {
                Err(())
            } else {
                Ok(digits)
            }
        })
        .map(|digits| {
            let mut key = [0; SECRET_KEY_BASE_BYTES];
            digits
                .chunks_exact(2)
                .map(|pair| pair[0] << 4 | pair[1])
                .enumerate()
                .for_each(|(i, digit)| key[i] = digit);
            key
        })
        .map_err(|_| {
            format!(
                "{} must be {} hex digits (0-9, A-F)",
                LEAF_SECRET_KEY_BASE,
                SECRET_KEY_BASE_BYTES * 2
            )
        })
}

fn from_hex(c: char) -> Result<u8, ()> {
    match c {
        '0'..='9' => Ok((c as u32 - 0x30) as u8),
        'A'..='F' => Ok((c as u32 - 0x41 + 10) as u8),
        'a'..='f' => Ok((c as u32 - 0x61 + 10) as u8),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex('0'), Ok(0));
        assert_eq!(from_hex('9'), Ok(9));
        assert_eq!(from_hex('a'), Ok(10));
        assert_eq!(from_hex('f'), Ok(15));
        assert_eq!(from_hex('A'), Ok(10));
        assert_eq!(from_hex('F'), Ok(15));

        assert_eq!(from_hex(' '), Err(()));
        assert_eq!(from_hex('z'), Err(()));
    }

    #[test]
    fn test_parse_secret_key_base() {
        assert_eq!(
            parse_secret_key_base(
                "0102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F20"
            ),
            Ok([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32
            ])
        );
        assert!(parse_secret_key_base("").is_err());
        // too long
        assert!(parse_secret_key_base(
            "0102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F2000"
        )
        .is_err());
        // too short
        assert!(parse_secret_key_base("CAFE").is_err());
        // non hex char
        assert!(parse_secret_key_base(
            "0102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F20ZZ"
        )
        .is_err());
    }
}
