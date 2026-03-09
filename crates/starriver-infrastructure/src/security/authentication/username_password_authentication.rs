use axum::http::StatusCode;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::security::authentication::core::{
    credential::Credential,
    principal::{Principal, SimpleAuthority},
};

/// 区别与用户提交的用户名&密码，该类型能包含更多的信心，比如IP等随HTTP请求携带的其他信息
#[derive(Clone, Debug)]
pub struct UsernamePasswordCredential {
    pub username: String,
    pub password: String,
}

impl Credential for UsernamePasswordCredential {}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
    #[serde(default)]
    pub authorities: Vec<SimpleAuthority>,
}

impl Principal for AuthenticatedUser {
    type Id = String;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let cookie_jar = CookieJar::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    error!("提取cookie失败");
                    StatusCode::UNAUTHORIZED
                })?;

            let id_cookie = cookie_jar.get("id").ok_or_else(|| {
                error!("缺少 `id` Cookie");
                StatusCode::UNAUTHORIZED
            })?;

            serde_json::from_str::<AuthenticatedUser>(id_cookie.value()).map_err(|e| {
                error!("解析cookie失败, {}", e);
                StatusCode::UNAUTHORIZED
            })
        }
    }
}
