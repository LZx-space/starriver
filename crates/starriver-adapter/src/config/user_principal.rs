use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use starriver_application::repository::user::user_repository::UserRepositoryImpl as DomainUserRepoImpl;
use starriver_domain::user::repository::UserRepository as DomainUserRepo;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use starriver_infrastructure::security::authentication::core::credential::{Credential, Ctx};
use starriver_infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use starriver_infrastructure::security::authentication::password_hasher::{
    from_hashed_password, verify_password,
};
use std::fmt::Debug;
use tracing::{error, warn};

#[derive(Clone)]
pub struct UsernamePasswordCredential {
    username: String,
    password: String,
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

impl Credential for UsernamePasswordCredential {
    fn request_details(&self) -> Ctx {
        Ctx {}
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    username: Username,
    #[serde(skip_serializing)]
    password: Password,
    #[serde(default)]
    authorities: Vec<SimpleAuthority>,
}

impl Principal for User {
    type Id = Username;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let cookie_jar = CookieJar::from_request(req, state).await.map_err(|_| {
                error!("提取cookie失败");
                StatusCode::UNAUTHORIZED
            })?;

            let id_cookie = cookie_jar.get("id").ok_or_else(|| {
                error!("缺少 `id` Cookie");
                StatusCode::UNAUTHORIZED
            })?;

            serde_json::from_str::<User>(id_cookie.value()).map_err(|e| {
                error!("解析cookie失败, {}", e);
                StatusCode::UNAUTHORIZED
            })
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
pub trait UserRepository {
    fn find_by_id(
        &self,
        user_id: &String,
    ) -> impl Future<Output = Result<User, AuthenticationError>> + Send;
}

#[derive(Clone)]
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
        let user = self.delegate.find_by_username(user_id).await.map_err(|e| {
            warn!("Failed to find user by username: {}", e);
            AuthenticationError::Unknown
        })?;
        match user {
            Some(u) => Ok(User {
                username: u.username,
                password: u.password,
                authorities: vec![],
            }),
            None => {
                warn!("User not found with username: {}", user_id);
                Err(AuthenticationError::UsernameNotFound)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
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

    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        let username = &credential.username;
        let password = &credential.password;
        async move {
            // 查找用户
            let user = self.user_repository.find_by_id(username).await?;
            // 验证密码
            let password_hash_string = from_hashed_password(user.password.hashed_password_string())
                .map_err(|e| {
                    error!(
                        "bad hashed password string in {} repository, {}",
                        username, e
                    );
                    AuthenticationError::BadPassword
                })?;
            verify_password(password.as_str(), &password_hash_string)
                .map(|_| user)
                .map_err(|e| {
                    error!("verify {} hashed password error: {}", username, e);
                    AuthenticationError::BadPassword
                })
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_user_serialize() {
        let password = Password::create_password("password").unwrap();
        let user = User {
            username: Username::new("username").unwrap(),
            password,
            authorities: vec![],
        };
        println!("{}", serde_json::to_string(&user).unwrap());
    }
}
