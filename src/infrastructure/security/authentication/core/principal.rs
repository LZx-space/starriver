use serde::{Deserialize, Serialize};

use crate::infrastructure::security::authentication::core::credentials::Credentials;

/// 认证主体
#[derive(Serialize, Deserialize)]
pub struct Principal<T: Credentials> {
    authenticated: bool,
    client_details: ClientDetails,
    credentials: T,
}

impl<T: Credentials> Principal<T> {
    /// create instance with [`credentials`] and [`client_details`]
    ///
    /// [`credentials`]: T
    /// [`client_details`]: ClientDetails
    pub fn new(credentials: T, client_details: ClientDetails) -> Principal<T> {
        Principal {
            authenticated: false,
            client_details,
            credentials,
        }
    }

    pub fn set_authenticated(&mut self) {
        self.authenticated = true
    }

    pub fn client_details(&self) -> &ClientDetails {
        &self.client_details
    }

    pub fn credentials(&self) -> &T {
        &self.credentials
    }
}

/// 认证请求的客户端的详情，记录HTTP协议中的其它信息
#[derive(Serialize, Deserialize)]
pub struct ClientDetails {}
