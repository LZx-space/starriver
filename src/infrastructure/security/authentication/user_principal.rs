use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use crate::infrastructure::security::authentication::core::proof::{Proof, RequestDetails};
use crate::infrastructure::security::authentication::util::{
    hash_password, to_password_hash_string_struct, verify_password,
};

pub struct UserProof {
    username: String,
    password: String,
}

impl Proof for UserProof {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn request_details(&self) -> RequestDetails {
        RequestDetails {}
    }
}

impl UserProof {
    pub fn new(username: String, password: String) -> Self {
        UserProof { username, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
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
    fn password(&self) -> &String {
        &self.password
    }
}

pub struct UserRepository {}

impl UserRepository {
    fn find_by_id(&self, user_id: &String) -> Option<User> {
        match hash_password("password", "ABCDEFGH") {
            Ok(hash_string) => Some(User {
                username: user_id.clone(),
                password: hash_string.to_string(),
            }),
            Err(_) => None,
        }
    }
}

pub struct UserAuthenticator {
    user_repository: UserRepository,
}

impl UserAuthenticator {
    pub fn new(repo: UserRepository) -> UserAuthenticator {
        UserAuthenticator {
            user_repository: repo,
        }
    }
}

impl Authenticator for UserAuthenticator {
    type Proof = UserProof;
    type Principal = User;

    fn prove(&self, proof: &Self::Proof) -> Result<Self::Principal, AuthenticationError> {
        let username = proof.id();
        let user = self.user_repository.find_by_id(username);
        match user {
            None => Err(AuthenticationError::UsernameNotFound),
            Some(user) => {
                let password_hash_string =
                    to_password_hash_string_struct(&user.password).map_err(|e| {
                        println!("{}'s password was not hashed: {}", user.username, e);
                        AuthenticationError::BadPassword
                    })?;
                let result = verify_password(proof.password.as_str(), password_hash_string);
                match result {
                    Ok(_) => Ok(user),
                    Err(_) => Err(AuthenticationError::BadPassword),
                }
            }
        }
    }
}
