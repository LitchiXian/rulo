use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_success() {
        let password = "my_secure_password";
        let hashed = hash_password(password).unwrap();
        assert!(hashed.starts_with("$argon2"));
        assert!(verify_password(password, &hashed).unwrap());
    }

    #[test]
    fn verify_wrong_password() {
        let hashed = hash_password("correct_password").unwrap();
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn hash_produces_different_salts() {
        let h1 = hash_password("same_password").unwrap();
        let h2 = hash_password("same_password").unwrap();
        assert_ne!(h1, h2); // 不同盐，同密码哈希值不同
    }

    #[test]
    fn verify_invalid_hash_format() {
        assert!(verify_password("pwd", "not_a_valid_hash").is_err());
    }
}
