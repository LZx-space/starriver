use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("User not found")]
    IdentifierNotFound,

    #[error("Identifier is empty")]
    IdentifierEmpty,

    #[error("Password is empty")]
    PasswordEmpty,

    #[error("Malformed authentication request")]
    MalformedRequest,

    #[error("Bad password")]
    BadPassword,

    ///// transient states ///////////////
    #[error("User account is locked")]
    UserLocked,

    ///// life cycle ////////////////////
    #[error("User account is disabled")]
    UserDisabled,

    #[error("User account is deleted")]
    UserDeleted,

    /////////////////////////
    #[error("Inner error: {message}")]
    InnerError { message: String },
}
