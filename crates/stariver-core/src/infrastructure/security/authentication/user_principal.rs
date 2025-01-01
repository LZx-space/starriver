use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::error;

use crate::domain::user::repository::UserRepository as DomainUserRepository;
use crate::infrastructure::repository::user::user_repository::UserRepositoryImpl as DomainUserRepoImpl;
use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::credential::{Credential, Ctx};
use crate::infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use crate::infrastructure::security::authentication::util::{
    to_password_hash_string_struct, verify_password,
};

pub struct UsernamePasswordCredential {
    username: String,
    password: String,
}

impl Credential for UsernamePasswordCredential {
    fn request_details(&self) -> Ctx {
        Ctx {}
    }
}

impl UsernamePasswordCredential {
    pub fn new(username: String, password: String) -> Result<Self, AuthenticationError> {
        if username.is_empty() {
            return Err(AuthenticationError::UsernameEmpty);
        }
        if password.is_empty() {
            return Err(AuthenticationError::PasswordEmpty);
        }
        Ok(UsernamePasswordCredential { username, password })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    #[serde(skip_serializing, default)]
    password: String,
    #[serde(default)]
    authorities: Vec<SimpleAuthority>,
}

impl Principal for User {
    type Id = String;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl User {
    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[trait_variant::make(HttpService: Send)]
pub trait UserRepository {
    async fn find_by_id(&self, user_id: &String) -> Result<User, AuthenticationError>;
}

pub struct UserRepositoryImpl {
    delegate: DomainUserRepoImpl,
}

impl UserRepositoryImpl {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        UserRepositoryImpl {
            delegate: DomainUserRepoImpl::new(conn),
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: &String) -> Result<User, AuthenticationError> {
        let result = self
            .delegate
            .find_by_username(user_id)
            .await
            .map_err(|e| AuthenticationError::Unknown);
        match result {
            Ok(op) => op
                .ok_or(AuthenticationError::UsernameNotFound)
                .map(|u| User {
                    username: u.username,
                    password: u.password,
                    authorities: vec![],
                }),
            Err(err) => Err(err),
        }
    }
}

pub struct UserAuthenticator {
    user_repository: UserRepositoryImpl,
}

impl UserAuthenticator {
    pub fn new(repo: UserRepositoryImpl) -> UserAuthenticator {
        UserAuthenticator {
            user_repository: repo,
        }
    }
}

impl Authenticator for UserAuthenticator {
    type Credential = UsernamePasswordCredential;
    type Principal = User;

    async fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        let username = &credential.username;
        let user = self.user_repository.find_by_id(username).await;
        match user {
            Ok(user) => {
                let password_hash_string = to_password_hash_string_struct(user.password())
                    .map_err(|e| {
                        error!(
                            "{}'s password was not hashed: {}",
                            user.username(),
                            e.to_string()
                        );
                        AuthenticationError::BadPassword
                    })?;
                let result = verify_password(credential.password.as_str(), password_hash_string);
                match result {
                    Ok(_) => Ok(user),
                    Err(_) => Err(AuthenticationError::BadPassword),
                }
            }
            Err(err) => Err(err),
        }
    }
}
