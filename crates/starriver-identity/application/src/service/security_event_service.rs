use starriver_identity_domain::security_event::{
    entity::SecurityEvent, repository::SecurityEventRepository, value_object::SecurityEventType,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::CtxError;

pub struct SecurityEventApplicationService<REPO: SecurityEventRepository> {
    repo: REPO,
}

impl<REPO: SecurityEventRepository> SecurityEventApplicationService<REPO> {
    pub fn new(repo: REPO) -> Self {
        Self { repo }
    }

    /// 记录一条安全事件
    pub async fn record(
        &self,
        user_id: Uuid,
        event_type: SecurityEventType,
        message: &str,
    ) -> Result<SecurityEvent, CtxError> {
        let event = SecurityEvent::new(user_id, event_type, message);
        self.repo.insert(event).await.map_err(CtxError::from)
    }

    /// 查询指定用户在时间窗口内的指定类型事件
    pub async fn find_by_user_since(
        &self,
        user_id: Uuid,
        event_type: SecurityEventType,
        since: OffsetDateTime,
    ) -> Result<Vec<SecurityEvent>, CtxError> {
        self.repo
            .find_by_user_id_since(user_id, event_type, since)
            .await
            .map_err(CtxError::from)
    }
}
