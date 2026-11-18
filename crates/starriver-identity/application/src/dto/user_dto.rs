pub mod input {
    use serde::{Deserialize, Serialize};
    use starriver_identity_domain::{
        error::DomainError,
        user::{
            event::UserSecurityEventKind,
            specification::{EmailSpec, UsernameSpec},
            value_object::{Email, Username},
        },
    };
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

    pub enum UserIdentifier {
        Username(Username),
        Email(Email),
    }

    impl UserIdentifier {
        pub fn parse(
            identifier: &str,
            username_spec: &UsernameSpec,
            email_spec: &EmailSpec,
        ) -> Result<UserIdentifier, DomainError> {
            const EMAIL_MARKER: char = '@';
            let trimmed = identifier.trim();
            if trimmed.matches(EMAIL_MARKER).count() == 1
                && !trimmed.starts_with(EMAIL_MARKER)
                && !trimmed.ends_with(EMAIL_MARKER)
            {
                let email = Email::new(trimmed, email_spec)?;
                Ok(Self::Email(email))
            } else {
                let username = Username::new(trimmed, username_spec)?;
                Ok(Self::Username(username))
            }
        }
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

    /// field are order by columns
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
