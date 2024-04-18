use crate::domain::tag::value_object::DomainType;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: Uuid,
    pub domain_type: DomainType,
    pub name: String,
}
