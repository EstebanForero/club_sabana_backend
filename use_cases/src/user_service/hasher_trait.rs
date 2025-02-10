use super::err::Result;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, content: &str) -> Result<String>;

    fn verify(&self, original: &str, hashed: &str) -> Result<bool>;
}
