use crate::shared_error::PasswordEncoderError;

pub trait PasswordEncoder {
    fn encode(&self, password: &str) -> core::result::Result<String, PasswordEncoderError>;
    fn verify(
        &self,
        raw_password: &str,
        encode_password: &str,
    ) -> core::result::Result<(), PasswordEncoderError>;
}
