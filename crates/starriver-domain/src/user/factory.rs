use crate::user::repository::UserRepository;

pub struct UserFactory<T> {
    repository: T,
}

impl<T> UserFactory<T>
where
    T: UserRepository,
{
    pub async fn new(repo: T) -> Self {
        UserFactory { repository: repo }
    }
}
