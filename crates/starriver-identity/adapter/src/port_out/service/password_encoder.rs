use starriver_identity_domain::common::traits::PasswordEncoder;

#[derive(Clone)]
pub struct DefaultPasswordEncoder {}

impl PasswordEncoder for DefaultPasswordEncoder {
    fn encode(
        &self,
        password: &str,
    ) -> core::result::Result<String, starriver_identity_domain::common::error::PasswordEncoderError>
    {
        todo!()
    }

    fn verify(
        &self,
        raw_password: &str,
        encode_password: &str,
    ) -> core::result::Result<(), starriver_identity_domain::common::error::PasswordEncoderError>
    {
        todo!()
    }
}
