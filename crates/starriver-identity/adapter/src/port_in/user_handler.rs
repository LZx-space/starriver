use axum::extract::State;
use axum::response::IntoResponse;

use starriver_identity_application::dto::user_dto::input::{
    ChangeMyPasswordCmd, ResetPasswordCmd, SendVerificationCodeCmd, UserRegisterCmd,
};
use starriver_shared_base::dto::OffsetPageQuery;
use starriver_shared_infra::extract::{Json, Query};
use starriver_shared_infra::middleware::authentication::default_impl::AuthenticatedJwtClaims;

use crate::api_error::ApiError;
use crate::port_in::state::IdentityState;

pub async fn me(user: AuthenticatedJwtClaims) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(user))
}

////////////////////////////////////////////////////////////////////

pub async fn paginate(
    state: State<IdentityState>,
    _: AuthenticatedJwtClaims,
    q: Query<OffsetPageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = state.paginate_interactor.paginate(q.0).await?;
    Ok(Json(page))
}

////////////////////////////////////////////////////////////////////

pub async fn send_email_verification_code(
    state: State<IdentityState>,
    cmd: Json<SendVerificationCodeCmd>,
) -> Result<impl IntoResponse, ApiError> {
    // 不会失败：错误已记录在用例内部（防枚举）
    let _ = state
        .register_interactor
        .send_register_email(&cmd.0.email)
        .await;
    Ok(())
}

// #[axum::debug_handler]
pub async fn register_user(
    state: State<IdentityState>,
    cmd: Json<UserRegisterCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = cmd.0;
    state.register_interactor.register_user(cmd).await?;
    Ok(())
}

////////////////////////////////////////////////////////////////////

pub async fn change_my_password(
    state: State<IdentityState>,
    user: AuthenticatedJwtClaims,
    cmd: Json<ChangeMyPasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let username = &user.username;
    state
        .change_my_password_interactor
        .change_my_password(username, cmd.0)
        .await?;
    Ok(())
}

pub async fn send_reset_password_verification_code(
    state: State<IdentityState>,
    cmd: Json<SendVerificationCodeCmd>,
) -> Result<impl IntoResponse, ApiError> {
    // 不会失败：错误已记录在用例内部（防枚举）
    let _ = state
        .reset_password_interactor
        .send_verification_code(&cmd.0.email)
        .await;
    Ok(())
}

pub async fn reset_password_with_verification_code(
    state: State<IdentityState>,
    cmd: Json<ResetPasswordCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .reset_password_interactor
        .reset_password_with_verification_code(cmd.0)
        .await?;
    Ok(())
}
