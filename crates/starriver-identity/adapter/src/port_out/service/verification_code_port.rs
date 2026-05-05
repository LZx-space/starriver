use starriver_identity_application::port_out::verification_code_port::VerificationCodePort;

#[derive(Clone)]
pub struct DefaultVerificationCodePort {}

impl VerificationCodePort for DefaultVerificationCodePort {
    async fn send_code(
        &self,
        email: &str,
    ) -> Result<(), starriver_identity_application::common::error::SendVerificationCodeError> {
        todo!()
    }

    async fn validate_code(
        &self,
        email: &str,
        code: &str,
    ) -> Result<(), starriver_identity_application::common::error::ValidateVerificationCodeError>
    {
        todo!()
    }
}
