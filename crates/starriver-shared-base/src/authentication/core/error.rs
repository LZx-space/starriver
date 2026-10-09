use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("username not found")]
    UsernameNotFound,

    #[error("username is empty")]
    UsernameEmpty,

    #[error("password not found")]
    PasswordNotFound,

    #[error("password is empty")]
    PasswordEmpty,

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
