use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use starriver_identity_domain::password_encoder::{EncodeError, PasswordEncoder, VerifyError};

#[derive(Clone, Default)]
pub struct Argon2PasswordEncoder {
    argon2: Argon2<'static>,
}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode(&self, password: &str) -> Result<String, EncodeError> {
        let hash = self
            .argon2
            .hash_password(password.as_bytes())
            .map_err(|e| EncodeError::Hashing(Box::new(e)))?;
        Ok(hash.to_string())
    }

    fn verify(&self, raw_password: &str, encoded_password: &str) -> Result<bool, VerifyError> {
        let password_hash = PasswordHash::new(encoded_password)
            .map_err(|e| VerifyError::Verification(Box::new(e)))?;
        match self
            .argon2
            .verify_password(raw_password.as_bytes(), &password_hash)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false), // 密码不匹配
            Err(e) => Err(VerifyError::Verification(Box::new(e))),
        }
    }
}
