use std::fmt::Debug;

use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::credentials::Credentials;
use crate::infrastructure::security::authentication::core::credentials_repository::CredentialsRepository;
use crate::infrastructure::security::authentication::core::principal::Principal;

/// 用户名密码类型的凭证
#[derive(Debug)]
pub struct UsernamePasswordCredentials {
    username: String,
    password: String,
}
impl UsernamePasswordCredentials {
    pub fn new(username: String, password: String) -> Self {
        UsernamePasswordCredentials { username, password }
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}

impl Credentials for UsernamePasswordCredentials {}

pub struct UserCredentialsRepository {}

impl CredentialsRepository for UserCredentialsRepository {
    type ID = String;
    type CredentialsType = UsernamePasswordCredentials;

    fn find_by_id(&self, credentials_id: &Self::ID) -> Option<Box<Self::CredentialsType>> {
        let credentials = UsernamePasswordCredentials {
            username: credentials_id.clone(),
            password: "password".to_string(),
        };
        Some(Box::new(credentials))
    }
}

pub struct UsernamePasswordCredentialsAuthenticator {
    credentials_repository:
        Box<dyn CredentialsRepository<ID = String, CredentialsType = UsernamePasswordCredentials>>,
}

impl UsernamePasswordCredentialsAuthenticator {
    pub fn new(
        repo: Box<
            dyn CredentialsRepository<ID = String, CredentialsType = UsernamePasswordCredentials>,
        >,
    ) -> UsernamePasswordCredentialsAuthenticator {
        UsernamePasswordCredentialsAuthenticator {
            credentials_repository: repo,
        }
    }
}

impl Authenticator<UsernamePasswordCredentials> for UsernamePasswordCredentialsAuthenticator {
    fn authenticate(
        &self,
        principal: &mut Principal<UsernamePasswordCredentials>,
    ) -> Result<(), AuthenticationError> {
        let credentials = principal.credentials();
        let credentials_in_repo = self
            .credentials_repository
            .find_by_id(&credentials.username);
        match credentials_in_repo {
            None => Err(AuthenticationError::UsernameNotFound),
            Some(credentials_in_repo) => {
                if credentials_in_repo.password == credentials.password {
                    principal.set_authenticated();
                    Ok(())
                } else {
                    Err(AuthenticationError::BadPassword)
                }
            }
        }
    }
}
