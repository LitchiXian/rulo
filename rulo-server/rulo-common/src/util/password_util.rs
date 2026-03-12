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
