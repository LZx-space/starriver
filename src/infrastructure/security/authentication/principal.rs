use std::ops::Deref;

use crate::infrastructure::security::authentication::credentials::Credentials;

/// 认证主体
pub struct Principal {
    authenticated: bool,
    client_details: ClientDetails,
    credentials: Box<dyn Credentials>,
}

impl Principal {
    /// create instance with [`credentials`] and [`client_details`]
    ///
    /// [`credentials`]: Credentials
    /// [`client_details`]: ClientDetails
    pub fn new(credentials: Box<dyn Credentials>, client_details: ClientDetails) -> Principal {
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

    pub fn credentials(&self) -> &dyn Credentials {
        self.credentials.deref()
    }
}

/// 认证请求的客户端的详情，记录HTTP协议中的其它信息
pub struct ClientDetails {}
