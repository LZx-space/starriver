use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts};
use axum_extra::extract::CookieJar;
use http::{StatusCode, request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use starriver_shared_base::authentication::PrincipalClaims;
use tracing::{error, info};

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub jws_secret: Arc<String>,
}

impl Auth {
    pub fn jws_secret_as_ref(&self) -> &[u8] {
        self.jws_secret.as_bytes()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
pub struct AuthenticatedUser(pub PrincipalClaims);

const AUTHENTION_TOKEN_COOKIE_NAME: &str = "token";

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Auth: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_infallible| StatusCode::UNAUTHORIZED)?;

        let jws = cookie_jar
            .get(AUTHENTION_TOKEN_COOKIE_NAME)
            .ok_or_else(|| {
                info!("authentication cookie not found in request");
                StatusCode::UNAUTHORIZED
            })?
            .value();

        let authentication = Auth::from_ref(state);

        decode::<PrincipalClaims>(
            jws,
            &DecodingKey::from_secret(authentication.jws_secret_as_ref()),
            &Validation::default(),
        )
        .map(|data| {
            let principal_claims = data.claims;
            AuthenticatedUser(principal_claims)
        })
        .map_err(|e| {
            error!(error = %e, "JWS token decode failed");
            StatusCode::UNAUTHORIZED
        })
    }
}
