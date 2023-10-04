use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use crate::infrastructure::security::authentication::core::proof::{Proof, RequestDetails};

pub struct UsernamePasswordProof {
    username: String,
    password: String,
}

impl Proof for UsernamePasswordProof {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn request_details() -> RequestDetails {
        RequestDetails {}
    }
}

impl UsernamePasswordProof {
    pub fn new(username: String, password: String) -> Self {
        UsernamePasswordProof { username, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsernamePasswordPrincipal {
    username: String,
    password: String,
}

impl Principal for UsernamePasswordPrincipal {
    type Id = String;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl UsernamePasswordPrincipal {
    fn password(&self) -> &String {
        &self.password
    }
}

pub struct UserPrincipalRepository {}

impl UserPrincipalRepository {
    fn find_by_id(&self, credentials_id: &String) -> Option<UsernamePasswordPrincipal> {
        let credentials = UsernamePasswordPrincipal {
            username: credentials_id.clone(),
            password: "password".to_string(),
        };
        Some(credentials)
    }
}

pub struct UsernamePasswordPrincipalAuthenticator {
    principal_repository: UserPrincipalRepository,
}

impl UsernamePasswordPrincipalAuthenticator {
    pub fn new(repo: UserPrincipalRepository) -> UsernamePasswordPrincipalAuthenticator {
        UsernamePasswordPrincipalAuthenticator {
            principal_repository: repo,
        }
    }
}

impl Authenticator for UsernamePasswordPrincipalAuthenticator {
    type Proof = UsernamePasswordProof;
    type Principal = UsernamePasswordPrincipal;

    fn prove(&self, proof: &Self::Proof) -> Result<Self::Principal, AuthenticationError> {
        let username = proof.id();
        let option = self.principal_repository.find_by_id(username);
        match option {
            None => Err(AuthenticationError::UsernameNotFound),
            Some(principal) => {
                if proof.password == principal.password {
                    Ok(principal)
                } else {
                    Err(AuthenticationError::BadPassword)
                }
            }
        }
    }
}
