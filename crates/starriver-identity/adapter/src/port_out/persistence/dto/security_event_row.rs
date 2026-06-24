use sea_orm::FromQueryResult;
use starriver_identity_application::dto::user_dto::{
    req::SecurityEventType, res::SecurityEventDto,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::security_event_po::SecurityEventTypePo;

#[derive(FromQueryResult)]
pub struct SecurityEventRow {
    pub id: Uuid,
    pub username: String,
    pub event_type: SecurityEventTypePo,
    pub occurred_at: OffsetDateTime,
}

impl From<SecurityEventRow> for SecurityEventDto {
    fn from(e: SecurityEventRow) -> Self {
        SecurityEventDto {
            id: e.id,
            username: e.username,
            event_type: SecurityEventType::from(e.event_type).to_string(),
            occurred_at: e.occurred_at,
        }
    }
}
