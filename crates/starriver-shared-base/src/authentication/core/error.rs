use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Identifier not found")]
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
    #[error("User locked")]
    UserLocked,

    ///// life cycle ////////////////////
    #[error("User disabled")]
    UserDisabled,

    #[error("User deleted")]
    UserDeleted,

    /////////////////////////
    #[error("Inner error: {message}")]
    InnerError { message: String },
}
