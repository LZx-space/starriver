use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use starriver_identity_application::port::user_repository::UserRepository;
use starriver_identity_domain::user::entity::User;
use starriver_shared_base::db::Revision;
use starriver_shared_base::error::RepositoryError;
use starriver_shared_framework::db::DefaultConnection;
use starriver_shared_framework::db::DefaultTransaction;
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;

use crate::port_out::persistence::po::user_po::ActiveModel;
use crate::port_out::persistence::po::user_po::Column;
use crate::port_out::persistence::po::user_po::Entity;

pub struct DefaultUserRepository;

impl DefaultUserRepository {
    async fn find_by_username(
        &self,
        conn: &impl ConnectionTrait,
        username: &str,
    ) -> Result<Option<User>, RepositoryError> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(conn)
            .await
            .map(|e| {
                e.map(|e| {
                    User::from_repo(
                        e.id,
                        e.username,
                        e.password,
                        e.email,
                        e.life_cycle.into(),
                        e.password_locked_until,
                        e.password_window_start,
                        e.password_attempts as u8,
                    )
                })
            })
            .map_err(db_2_repo_error)
    }

    async fn insert(
        &self,
        conn: &impl ConnectionTrait,
        user: User,
    ) -> Result<User, RepositoryError> {
        let (
            id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
        ) = user.dissolve();
        ActiveModel {
            id: Set(id),
            username: Set(username.to_string()),
            password: Set(password.as_str().to_string()),
            email: Set(email.to_string()),
            life_cycle: Set(life_cycle.into()),
            password_locked_until: Set(password_locked_until),
            password_window_start: Set(password_window_start),
            password_attempts: Set(password_attempts as i16),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|m| {
            User::from_repo(
                m.id,
                m.username,
                m.password,
                m.email,
                m.life_cycle.into(),
                m.password_locked_until,
                m.password_window_start,
                m.password_attempts as u8,
            )
        })
    }

    async fn delete(
        &self,
        conn: &impl ConnectionTrait,
        user_id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(user_id)
            .exec(conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(db_2_repo_error)
    }

    async fn update(
        &self,
        conn: &impl ConnectionTrait,
        user: Revision<User>,
    ) -> Result<User, RepositoryError> {
        let (original, modified) = user.dissolve();
        let (
            user_id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
        ) = original.dissolve();
        let (
            _,
            new_username,
            new_password,
            new_email,
            new_life_cycle,
            new_password_locked_until,
            new_password_window_start,
            new_password_attempts,
        ) = modified.dissolve();

        let mut username = Unchanged(username.as_str().to_string());
        username.set_if_not_equals(new_username.as_str().to_string());
        let mut password = Unchanged(password.as_str().to_string());
        password.set_if_not_equals(new_password.as_str().to_string());
        let mut email = Unchanged(email.to_string());
        email.set_if_not_equals(new_email.to_string());
        let mut life_cycle = Unchanged(life_cycle.into());
        life_cycle.set_if_not_equals(new_life_cycle.into());
        let mut password_locked_until = Unchanged(password_locked_until);
        password_locked_until.set_if_not_equals(new_password_locked_until);
        let mut password_window_start = Unchanged(password_window_start);
        password_window_start.set_if_not_equals(new_password_window_start);
        let mut password_attempts = Unchanged(password_attempts as i16);
        password_attempts.set_if_not_equals(new_password_attempts as i16);

        ActiveModel {
            id: Unchanged(user_id),
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|m| {
            User::from_repo(
                m.id,
                m.username,
                m.password,
                m.email,
                m.life_cycle.into(),
                m.password_locked_until,
                m.password_window_start,
                m.password_attempts as u8,
            )
        })
    }
}

impl UserRepository<DefaultConnection> for DefaultUserRepository {
    async fn find_by_username(
        &self,
        conn: &DefaultConnection,
        username: &str,
    ) -> Result<Option<User>, RepositoryError> {
        self.find_by_username(conn, username).await
    }

    async fn insert(&self, conn: &DefaultConnection, user: User) -> Result<User, RepositoryError> {
        self.insert(conn, user).await
    }

    async fn update(
        &self,
        conn: &DefaultConnection,
        user: Revision<User>,
    ) -> Result<User, RepositoryError> {
        self.update(conn, user).await
    }

    async fn delete(
        &self,
        conn: &DefaultConnection,
        user_id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        self.delete(conn, user_id).await
    }
}

impl UserRepository<DefaultTransaction> for DefaultUserRepository {
    async fn find_by_username(
        &self,
        conn: &DefaultTransaction,
        username: &str,
    ) -> Result<Option<User>, RepositoryError> {
        self.find_by_username(conn, username).await
    }

    async fn insert(&self, conn: &DefaultTransaction, user: User) -> Result<User, RepositoryError> {
        self.insert(conn, user).await
    }

    async fn update(
        &self,
        conn: &DefaultTransaction,
        user: Revision<User>,
    ) -> Result<User, RepositoryError> {
        self.update(conn, user).await
    }

    async fn delete(
        &self,
        conn: &DefaultTransaction,
        user_id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        self.delete(conn, user_id).await
    }
}
