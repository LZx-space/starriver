use starriver_blogging_domain::post::{entity::Post, repository::PostRepository};
use starriver_shared_base::{error::RepositoryError, repository::Revision};

pub struct DefaultPostRepository {}

impl PostRepository for DefaultPostRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Post>, RepositoryError> {
        todo!()
    }

    async fn add(&self, post: Post) -> Result<Post, RepositoryError> {
        todo!()
    }

    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<bool, RepositoryError> {
        todo!()
    }

    async fn update(&self, post: Revision<Post>) -> Result<Post, RepositoryError> {
        todo!()
    }
}
