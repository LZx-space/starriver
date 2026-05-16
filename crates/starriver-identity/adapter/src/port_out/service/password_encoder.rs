use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHashString, SaltString, rand_core::OsRng},
};
use starriver_identity_domain::{
    password_encoder::PasswordEncoder, shared_error::PasswordEncoderError,
};

#[derive(Clone, Default)]
pub struct Argon2PasswordEncoder {
    argon2: Argon2<'static>,
}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode(&self, password: &str) -> core::result::Result<String, PasswordEncoderError> {
        let salt_string = SaltString::generate(&mut OsRng);
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| PasswordEncoderError::EncodingFailed(e.to_string()))?;
        Ok(hash.serialize().to_string())
    }

    fn verify(
        &self,
        raw_password: &str,
        encode_password: &str,
    ) -> core::result::Result<(), PasswordEncoderError> {
        let password_hash_str = PasswordHashString::new(encode_password)
            .map_err(|e| PasswordEncoderError::InternalError(e.to_string()))?;
        let password_hash = password_hash_str.password_hash();
        self.argon2
            .verify_password(raw_password.as_bytes(), &password_hash)
            .map_err(|e| PasswordEncoderError::VerificationFailed(e.to_string()))
    }
}
