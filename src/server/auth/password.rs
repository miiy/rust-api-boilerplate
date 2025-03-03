use super::error::AuthError;
use rs_crypto::argon2;

pub fn argon2_hash(password: &str) -> Result<String, AuthError> {
    Ok(argon2::hash_password(password)?)
}

pub fn argon2_verify(password: &str, hash: &str) -> Result<bool, AuthError> {
    Ok(argon2::verify_password(password, hash)?)
}
