use crate::domain::tag::value_object::DomainType;
use uuid::Uuid;

#[derive(Debug)]
pub struct Tag {
    pub id: Uuid,
    pub domain_type: DomainType,
    pub name: String,
}
