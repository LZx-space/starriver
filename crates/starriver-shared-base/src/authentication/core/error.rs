use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("identifier not found")]
    IdentifierNotFound,

    #[error("identifier is empty")]
    IdentifierEmpty,

    #[error("password is empty")]
    PasswordEmpty,

    #[error("malformed authentication request")]
    MalformedRequest,

    #[error("bad password")]
    BadPassword,

    ///// transient states ///////////////
    #[error("user locked")]
    UserLocked,

    ///// life cycle ////////////////////
    #[error("user disabled")]
    UserDisabled,

    #[error("user deleted")]
    UserDeleted,

    /////////////////////////
    #[error("inner error: {message}")]
    InnerError { message: String },
}
