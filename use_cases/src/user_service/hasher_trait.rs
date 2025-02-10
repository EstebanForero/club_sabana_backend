use super::err::Result;

pub trait PasswordHasher {
    fn hash(content: &str) -> Result<&str>;

    fn verify(original: &str, hashed: &str) -> Result<bool>;
}
