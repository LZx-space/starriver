use axum::extract::State;
use axum::response::IntoResponse;

use starriver_identity_application::dto::user_dto::req::{
    ChangeMyPasswordCmd, ResetPasswordCmd, SendVerificationCodeCmd, UserRegisterCmd,
};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_framework::extract::{Json, JsonEx, Query};
use starriver_shared_framework::middleware::authentication::default_impl::AuthenticatedJwtClaims;
use starriver_shared_framework::response::ApiError;

use crate::error_mapping::map_error;
use crate::port_in::state::IdentityState;

pub async fn me(user: AuthenticatedJwtClaims) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(user))
}

////////////////////////////////////////////////////////////////////

pub async fn paginate(
    state: State<IdentityState>,
    _: AuthenticatedJwtClaims,
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
    cmd: Json<SendVerificationCodeCmd>,
) -> Result<impl IntoResponse, ApiError> {
    // 不会失败：错误已记录在用例内部（防枚举）
    let _ = state
        .user_interactor
        .send_register_email(&cmd.0.email)
        .await;
    Ok(())
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
    user: AuthenticatedJwtClaims,
    cmd: JsonEx<ChangeMyPasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let username = &user.username;
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
    // 不会失败：错误已记录在用例内部（防枚举）
    let _ = state
        .user_interactor
        .send_verification_code(&cmd.0.email)
        .await;
    Ok(())
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
