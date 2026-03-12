use jsonwebtoken::{DecodingKey, DecodingKeyKind, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"rulo_jwt_secret_key";
// 1 天
const EXPIRE_SECONDS: u64 = 1 * 24 * 3600;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // user_id
    pub sub: i64,
    // expire_time(unix timestamp)
    pub exp: u64,
}

pub fn generate_token(user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + EXPIRE_SECONDS;

    let claims = Claims { sub: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
