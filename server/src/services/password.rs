use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

pub struct Password;

impl Password {
    pub fn hash_password(password: &str) -> Result<String, BcryptError> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
        let valid = verify(password, hashed_password)?;
        Ok(valid)
    }
}
