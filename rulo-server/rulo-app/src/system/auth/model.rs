use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthUserDto {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}
