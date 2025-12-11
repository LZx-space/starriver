use crate::config::user_principal::User;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use tracing::error;

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
