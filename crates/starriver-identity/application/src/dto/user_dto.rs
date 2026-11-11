pub mod input {
    use serde::{Deserialize, Serialize};
    use starriver_identity_domain::user::event::UserSecurityEventKind;
    use starriver_shared_base::dto::CursorPageQuery;
    use time::OffsetDateTime;
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct UserRegisterCmd {
        pub username: String,
        pub password: String,
        pub email: String,
        #[validate(length(equal = 6))]
        pub verification_code: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct ChangeMyPasswordCmd {
        pub cur_password: String,
        pub new_password: String,
        pub new_password_confirm: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct SendVerificationCodeCmd {
        pub email: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct ResetPasswordCmd {
        pub email: String,
        #[validate(length(equal = 6))]
        pub verification_code: String,
        pub new_password: String,
        pub new_password_confirm: String,
    }
    //////////////////////////////////////////////////////////////////////////////////////

    /// 安全事件列表查询（游标分页）
    #[derive(Deserialize, Validate)]
    pub struct SecurityEventQuery {
        #[serde(flatten)]
        #[validate(nested)]
        pub pagination: CursorPageQuery<SecurityEventCursor>,
        pub user_id: Option<Uuid>,
        pub kind: Option<UserSecurityEventKind>,
        /// example: 2026-11-01T05:49:04.3137462Z
        #[serde(with = "time::serde::rfc3339::option", default)]
        pub from: Option<OffsetDateTime>,
        /// example: 2026-11-01T05:49:04.3137462Z
        #[serde(with = "time::serde::rfc3339::option", default)]
        pub to: Option<OffsetDateTime>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SecurityEventCursor {
        pub occurred_at: OffsetDateTime,
        pub id: Uuid,
    }
}

pub mod output {
    use serde::Serialize;
    use starriver_identity_domain::user::{
        event::UserSecurityEventKind, value_object::AccountStatus,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct UserDetail {
        pub id: Uuid,
        pub username: String,
        pub email: String,
    }

    #[derive(Serialize)]
    pub struct UserDetailDto {
        pub id: Uuid,
        pub username: String,
        pub email: String,
        pub status: AccountStatus,
        pub password_locked_until: Option<OffsetDateTime>,
        pub password_window_start: Option<OffsetDateTime>,
        pub password_attempts: i16,
    }

    #[derive(Serialize)]
    pub struct SecurityEventDto {
        pub id: Uuid,
        pub user_id: Uuid,
        pub username: String,
        pub kind: UserSecurityEventKind,
        pub occurred_at: OffsetDateTime,
    }
}
