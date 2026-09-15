use axum::extract::State;
use axum::response::IntoResponse;

use starriver_identity_application::dto::user_dto::req::{
    ChangeMyPasswordCmd, ResetPasswordCmd, SendVerificationCodeCmd, UserRegisterCmd,
    UserRegisterEmailCmd,
};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_base::middleware::authentication::core::principal::Principal;
use starriver_shared_framework::extract::{Json, JsonEx, Query};
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
    _: AuthenticatedUser,
    q: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_interactor
        .paginate(q.0)
        .await
        .map_err(map_error)
        .map(Json)
}

////////////////////////////////////////////////////////////////////

pub async fn send_email_verification_code(
    state: State<IdentityState>,
    cmd: Json<UserRegisterEmailCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_interactor
        .send_register_email(cmd.0)
        .await
        .map_err(|e| e.into())
}

#[axum::debug_handler]
pub async fn register_user(
    state: State<IdentityState>,
    cmd: JsonEx<UserRegisterCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = cmd.0;
    state
        .user_interactor
        .register_user(cmd)
        .await
        .map_err(map_error)
}

////////////////////////////////////////////////////////////////////

pub async fn change_my_password(
    state: State<IdentityState>,
    user: AuthenticatedUser,
    cmd: JsonEx<ChangeMyPasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let username = user.id();
    state
        .user_interactor
        .change_my_password(username, cmd.0)
        .await
        .map_err(map_error)
}

pub async fn send_reset_password_verification_code(
    state: State<IdentityState>,
    cmd: Json<SendVerificationCodeCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_interactor
        .send_verification_code(&cmd.0.identifier)
        .await
        .map_err(map_error)
}

pub async fn reset_password_with_verification_code(
    state: State<IdentityState>,
    cmd: JsonEx<ResetPasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_interactor
        .reset_password_with_verification_code(cmd.0)
        .await
        .map_err(map_error)
}
