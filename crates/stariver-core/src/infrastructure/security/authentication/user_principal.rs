use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::domain::user::repository::UserRepository as DomainUserRepository;
use crate::infrastructure::repository::user::user_repository::UserRepositoryImpl as DomainUserRepoImpl;
use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::credential::{
    Credential, RequestDetails,
};
use crate::infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use crate::infrastructure::security::authentication::util::{
    to_password_hash_string_struct, verify_password,
};

pub struct UserCredential {
    username: String,
    password: String,
}

impl Credential for UserCredential {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn request_details(&self) -> RequestDetails {
        RequestDetails {}
    }
}

impl UserCredential {
    pub fn new(username: String, password: String) -> Self {
        UserCredential { username, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
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
    pub(crate) delegate: DomainUserRepoImpl,
}

impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: &String) -> Result<User, AuthenticationError> {
        self.delegate
            .find_by_username(user_id)
            .await
            .map(|u| User {
                username: u.username,
                password: u.password,
                authorities: vec![],
            })
            .map_err(|e| AuthenticationError::UsernameNotFound)
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
    type Credential = UserCredential;
    type Principal = User;

    async fn do_authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        let username = credential.id();
        let user = self.user_repository.find_by_id(username).await;
        match user {
            Ok(user) => {
                let password_hash_string = to_password_hash_string_struct(user.password())
                    .map_err(|e| {
                        println!("{}'s password was not hashed: {}", user.username(), e);
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
