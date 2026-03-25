use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // user_id
    pub sub: i64,
    // expire_time(unix timestamp)
    pub exp: u64,
}

pub fn generate_token(user_id: i64, secret: &str, expire_hours: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + expire_hours * 3600;

    let claims = Claims { sub: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test_secret_key_for_unit_test";

    #[test]
    fn generate_and_verify_token() {
        let user_id = 12345i64;
        let token = generate_token(user_id, TEST_SECRET, 1).unwrap();
        let claims = verify_token(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn verify_with_wrong_secret_fails() {
        let token = generate_token(1, TEST_SECRET, 1).unwrap();
        assert!(verify_token(&token, "wrong_secret").is_err());
    }

    #[test]
    fn expired_token_fails() {
        // 构造一个 exp 在过去 2 分钟的 token，确保超过 jsonwebtoken 默认 60s leeway
        let past_exp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 120;
        let claims = Claims { sub: 1, exp: past_exp };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
        )
        .unwrap();
        assert!(verify_token(&token, TEST_SECRET).is_err());
    }

    #[test]
    fn malformed_token_fails() {
        assert!(verify_token("not.a.token", TEST_SECRET).is_err());
    }
}
