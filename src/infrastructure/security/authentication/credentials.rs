use std::collections::HashMap;

/// 凭证
pub trait Credentials {
    /// 凭证类型
    fn credentials_type(&self) -> CredentialsType;
    /// 不同类型的凭证会拥有不同的内容，比如“用户名密码”类型凭证拥有用户名和密码2个数据，OAuth2令牌只拥有一项内容
    fn content(&self) -> HashMap<&str, String>;
}

/// 凭证类型
#[derive(PartialEq)]
pub enum CredentialsType {
    /// 用户名&密码
    UsernamePassword,
}

// -------------------------

/// 用户名密码类型的凭证
pub struct UsernamePasswordCredentials {
    username: String,
    password: String,
}

impl Credentials for UsernamePasswordCredentials {
    fn credentials_type(&self) -> CredentialsType {
        CredentialsType::UsernamePassword
    }

    fn content(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        map.insert("username", self.username.clone());
        map.insert("password", self.password.clone());
        map
    }
}
