use sea_orm::FromQueryResult;
use starriver_identity_application::dto::user_dto::output::SecurityEventDto;
use starriver_identity_domain::user::event::UserSecurityEventKind;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::security_event_po::SecurityEventKindDb;

#[derive(FromQueryResult)]
pub struct SecurityEventRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub kind: SecurityEventKindDb,
    pub occurred_at: OffsetDateTime,
}

impl From<SecurityEventRow> for SecurityEventDto {
    fn from(e: SecurityEventRow) -> Self {
        SecurityEventDto {
            id: e.id,
            user_id: e.user_id,
            username: e.username,
            kind: UserSecurityEventKind::from(e.kind),
            occurred_at: e.occurred_at,
        }
    }
}
