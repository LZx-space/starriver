use axum::extract::State;
use axum::response::IntoResponse;

use starriver_identity_application::dto::user_dto::req::{
    ChangePasswordCmd, EmailActiveCmd, EmailVerifyCmd, UserActiveCmd, UserCmd,
};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_base::middleware::authentication::core::principal::Principal;
use starriver_shared_framework::extract::{Json, JsonEx, Path};
use starriver_shared_framework::middleware::authentication::default_impl::AuthenticatedUser;
use starriver_shared_framework::response::ApiError;

use crate::error_mapping::map_error;
use crate::port_in::state::IdentityState;

pub async fn me(user: AuthenticatedUser) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(user))
}

////////////////////////////////////////////////////////////////////

pub async fn paginate(
    state: State<IdentityState>,
    q: Json<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .paginate(q.0)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn send_register_email(
    state: State<IdentityState>,
    cmd: Json<EmailVerifyCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .send_register_email(cmd.0)
        .await
        .map_err(|e| e.into())
}

#[axum::debug_handler]
pub async fn register_user(
    state: State<IdentityState>,
    cmd: JsonEx<UserCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = cmd.0;
    state
        .user_service
        .register_user(cmd)
        .await
        .map_err(map_error)
}

////////////////////////////////////////////////////////////////////

pub async fn send_activate_email(
    state: State<IdentityState>,
    cmd: Json<EmailActiveCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .send_active_email(cmd.0)
        .await
        .map_err(|e| e.into())
}

pub async fn activate_user(
    state: State<IdentityState>,
    username: Path<String>,
    cmd: Json<UserActiveCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .activate_user(username.0, cmd.0)
        .await
        .map_err(map_error)
}

pub async fn change_password(
    state: State<IdentityState>,
    user: AuthenticatedUser,
    cmd: JsonEx<ChangePasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let username = user.id();
    state
        .user_service
        .change_password(username, cmd.0)
        .await
        .map_err(map_error)
}
