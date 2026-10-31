use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::ExprTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryTrait;
use sea_orm::sea_query::Expr;
use starriver_identity_application::port::user_repository::UserRepository;
use starriver_identity_domain::user::entity::User;
use starriver_identity_domain::user::value_object::Email;
use starriver_shared_base::db::{Executor, Revision};
use starriver_shared_base::error::RepositoryError;
use starriver_shared_infra::error_mapping::db_2_repo_error;
use time::OffsetDateTime;

use crate::port_out::persistence::po::user_po;
use crate::port_out::persistence::po::user_po::ActiveModel;
use crate::port_out::persistence::po::user_po::Column;
use crate::port_out::persistence::po::user_po::Entity;

pub struct DefaultUserRepository;

impl<T> UserRepository<T> for DefaultUserRepository
where
    T: Executor + ConnectionTrait,
{
    async fn find_by_username(
        &self,
        conn: &T,
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

    async fn find_by_email(&self, conn: &T, email: &str) -> Result<Option<User>, RepositoryError> {
        let email = Email::normalize(email);
        Entity::find()
            .filter(Column::Email.eq(email.as_str()))
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

    async fn insert(&self, conn: &T, user: User) -> Result<User, RepositoryError> {
        let (
            id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
        ) = user.into_parts();
        ActiveModel {
            id: Set(id),
            username: Set(username.into_inner()),
            password: Set(password.into_inner()),
            email: Set(email.into_inner()),
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

    async fn delete(&self, conn: &T, user_id: uuid::Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(user_id)
            .exec(conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(db_2_repo_error)
    }

    async fn update(&self, conn: &T, user: Revision<User>) -> Result<User, RepositoryError> {
        if !user.is_changed() {
            return Ok(user.into_original());
        }

        let (original, modified) = user.into_parts();
        let (
            user_id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
        ) = original.into_parts();
        let (
            _user_id,
            new_username,
            new_password,
            new_email,
            new_life_cycle,
            new_password_locked_until,
            new_password_window_start,
            new_password_attempts,
        ) = modified.into_parts();

        let window_changed = password_window_start != new_password_window_start;

        let mut username = Unchanged(username.into_inner());
        username.set_if_not_equals(new_username.into_inner());
        let mut password = Unchanged(password.into_inner());
        password.set_if_not_equals(new_password.into_inner());
        let mut email = Unchanged(email.into_inner());
        email.set_if_not_equals(new_email.into_inner());
        let mut life_cycle = Unchanged(life_cycle.into());
        life_cycle.set_if_not_equals(new_life_cycle.into());
        let mut password_locked_until = Unchanged(password_locked_until);
        password_locked_until.set_if_not_equals(new_password_locked_until);
        let mut password_window_start = Unchanged(password_window_start);
        password_window_start.set_if_not_equals(new_password_window_start);

        let model = ActiveModel {
            id: Unchanged(user_id),
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts: NotSet,
            created_at: NotSet,
            updated_at: Set(OffsetDateTime::now_utc()),
        };

        // 保证原子性，避免事务1更新前事务2拿到相同值然后设置一个相同值
        // 当前方案：但是N并发一起请求，其中任一流程在运行update前可能N-1都等着update，全部update完可能出现password_attempts > 边界值
        // 行锁方案：让连接池和数据库承压，高并发连接挂起影响其它查询，并且数据库压力大
        let attempts_expr_opt = if password_attempts == new_password_attempts {
            None
        } else if new_password_attempts == 0 {
            // 认证成功/改密成功：清零
            Some(Expr::value(0i16))
        } else if window_changed {
            // 新窗口：域已经把计数重置为 1，不能拿数据库里的旧窗口计数 +1，
            // 否则窗口过期后的第一次失败会把上一窗口的计数带进来，导致提前锁定。
            Some(Expr::value(new_password_attempts as i16))
        } else {
            // 同一窗口内的并发失败：不采用实体业务计算值，而在数据库当前值上原子 +1
            Some(Expr::col(Column::PasswordAttempts).add(1))
        };

        user_po::Entity::update_many()
            .set(model)
            .apply_if(attempts_expr_opt, |update, expr| {
                update.col_expr(Column::PasswordAttempts, expr)
            })
            .filter(Column::Id.eq(user_id))
            .exec_with_returning(conn)
            .await
            .map_err(db_2_repo_error)?
            .into_iter()
            .next()
            .ok_or_else(|| db_2_repo_error(DbErr::RecordNotFound("user not found".into())))
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
