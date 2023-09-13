use crate::domain::tag::aggregate::Tag;
use crate::domain::tag::repository::TagRepository;
use crate::domain::tag::value_object::DomainType;
use uuid::Uuid;

pub struct TagApplication<T> {
    pub repo: T,
}

impl<T> TagApplication<T>
where
    T: TagRepository,
{
    pub fn new(repo: T) -> Self {
        TagApplication { repo }
    }

    pub fn list_by_related_entity_id(&self, domain_type: DomainType, id: Uuid) -> Vec<Tag> {
        todo!()
    }
}
