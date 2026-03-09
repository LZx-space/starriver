use std::sync::Arc;
use std::time::Duration;

use starriver_application::user_service::UserApplication;
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use starriver_infrastructure::security::authentication::username_password_authentication::{
    AuthenticatedUser, UsernamePasswordCredential,
};
use starriver_infrastructure::security::authentication::web::timing_attack_protection::TimingAttackProtection;
use tokio::time::sleep;

pub struct UsernamePasswordAuthenticator {
    pub user_service: Arc<UserApplication>,
}

impl Authenticator for UsernamePasswordAuthenticator {
    type Credential = UsernamePasswordCredential;
    type Principal = AuthenticatedUser;

    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        async move { self.user_service.authenticate(credential).await }
    }
}

/// 异步运行时为tokio时，使用tokio的sleep函数实现延时以防止认证时的时差攻击
pub struct TokioTimingAttackProtection {
    pub delay: Duration,
}

impl TimingAttackProtection for TokioTimingAttackProtection {
    async fn delay(&self, already_spend: Duration) {
        let to_sleep = self.delay.saturating_sub(already_spend);
        if Duration::ZERO.eq(&to_sleep) {
            return;
        }
        sleep(to_sleep).await;
    }
}
