use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use starriver_identity_application::common::error::AppError;
use tracing::{error, warn};
use uuid::Uuid;

use crate::config::AuthConfig;

pub struct ApiError(AppError);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        ApiError(value)
    }
}

///////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct PrincipalClaims {
    exp: i64,  // Expiration time (as UTC timestamp)
    nbf: i64,  // Not Before (as UTC timestamp)
    iat: i64,  // Issued at (as UTC timestamp)
    sub: Uuid, // Subject (whom token refers to)
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

const AUTHENTION_TOKEN_COOKIE_NAME: &str = "token";

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // FromRef
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_infallible| StatusCode::UNAUTHORIZED)?;

        let jws = cookie_jar
            .get(AUTHENTION_TOKEN_COOKIE_NAME)
            .ok_or_else(|| {
                warn!("authentication cookie not found in request");
                StatusCode::UNAUTHORIZED
            })?
            .value();

        let authentication = AuthConfig::from_ref(state);

        decode::<PrincipalClaims>(
            jws,
            &DecodingKey::from_secret(authentication.jws_secret_as_ref()),
            &Validation::default(),
        )
        .map(|data| {
            let principal_claims = data.claims;
            AuthenticatedUser {
                id: principal_claims.sub,
                username: principal_claims.username,
                email: principal_claims.email,
            }
        })
        .map_err(|e| {
            error!(error = %e, "JWS token decode failed");
            StatusCode::UNAUTHORIZED
        })
    }
}
