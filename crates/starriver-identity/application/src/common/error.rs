use starriver_identity_domain::common::error::{
    DomainError, PasswordEncoderError, RepositoryError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {}

#[derive(Debug, Error)]
pub enum UserQueryError {}

#[derive(Debug, Error)]
pub enum SendVerificationCodeError {}

#[derive(Debug, Error)]
pub enum ValidateVerificationCodeError {}

///////////////////////////////////////////

impl From<UserQueryError> for AppError {
    fn from(value: UserQueryError) -> Self {
        todo!()
    }
}

impl From<SendVerificationCodeError> for AppError {
    fn from(value: SendVerificationCodeError) -> Self {
        todo!()
    }
}

impl From<ValidateVerificationCodeError> for AppError {
    fn from(value: ValidateVerificationCodeError) -> Self {
        todo!()
    }
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        todo!()
    }
}

impl From<RepositoryError> for AppError {
    fn from(value: RepositoryError) -> Self {
        todo!()
    }
}

impl From<PasswordEncoderError> for AppError {
    fn from(value: PasswordEncoderError) -> Self {
        todo!()
    }
}
