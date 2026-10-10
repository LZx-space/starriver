use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use starriver_identity_domain::{error::PasswordEncoderError, password_encoder::PasswordEncoder};

#[derive(Clone, Default)]
pub struct Argon2PasswordEncoder {
    argon2: Argon2<'static>,
}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode(&self, password: &str) -> Result<String, PasswordEncoderError> {
        let hash = self
            .argon2
            .hash_password(password.as_bytes())
            .map_err(|e| PasswordEncoderError::EncodingFailed(e.to_string()))?;
        Ok(hash.to_string())
    }

    fn verify(
        &self,
        raw_password: &str,
        encoded_password: &str,
    ) -> Result<bool, PasswordEncoderError> {
        let password_hash = PasswordHash::new(encoded_password)
            .map_err(|e| PasswordEncoderError::VerificationFailed(e.to_string()))?;
        match self
            .argon2
            .verify_password(raw_password.as_bytes(), &password_hash)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false), // 密码不匹配
            Err(e) => Err(PasswordEncoderError::VerificationFailed(e.to_string())),
        }
    }
}
