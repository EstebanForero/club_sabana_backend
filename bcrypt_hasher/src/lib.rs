use bcrypt::{hash, verify, DEFAULT_COST};
use use_cases::user_service::err::Error::*;
use use_cases::user_service::hasher_trait::PasswordHasher;

pub struct BcryptHasher;

impl PasswordHasher for BcryptHasher {
    fn hash(&self, content: &str) -> use_cases::user_service::err::Result<String> {
        let hash_str = hash(content, DEFAULT_COST).map_err(|err| ErrorHashing(format!("{err}")))?;

        Ok(hash_str)
    }

    fn verify(&self, original: &str, hashed: &str) -> use_cases::user_service::err::Result<bool> {
        let is_valid = verify(original, hashed).map_err(|err| ErrorHashing(format!("{err}")))?;

        Ok(is_valid)
    }
}
