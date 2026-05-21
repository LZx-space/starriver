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

    /////////////////////////
    #[error("user locked")]
    UserLocked,

    #[error("user disabled")]
    UserDisabled,

    /////////////////////////
    #[error("inner error: {message}")]
    InnerError {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
