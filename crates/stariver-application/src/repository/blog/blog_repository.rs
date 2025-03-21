use super::po::blog::ActiveModel;
use super::po::blog::Column;
use super::po::blog::Entity;
use anyhow::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};
use stariver_domain::blog::aggregate::Blog;
use stariver_domain::blog::repository::BlogRepository;
use stariver_infrastructure::model::blog::BlogPreview;
use stariver_infrastructure::model::page::{PageQuery, PageResult};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct BlogRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

impl BlogRepository for BlogRepositoryImpl {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<BlogPreview>, Error> {
        let blogs = Entity::find()
            .select_only()
            .columns([Column::Id, Column::Title, Column::CreateAt])
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .into_model::<BlogPreview>()
            .all(self.conn)
            .await?;
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .count(self.conn)
            .await?;
        Ok(PageResult::new(q.page, q.page_size, record_total, blogs))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, Error> {
        Entity::find_by_id(id)
            .one(self.conn)
            .await
            .map(|op| {
                op.map(|e| Blog {
                    id,
                    title: e.title.clone(),
                    body: e.body.clone(),
                    state: e.state.into(),
                    blogger_id: e.blogger_id,
                    create_at: e.create_at,
                    update_at: e.update_at,
                })
            })
            .map_err(Error::from)
    }

    async fn add(&self, e: Blog) -> Result<Blog, Error> {
        ActiveModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            state: Set(Default::default()),
            blogger_id: Set(e.blogger_id),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: Set(None),
        }
        .insert(self.conn)
        .await
        .map(|e| Blog {
            id: e.id,
            title: e.title,
            body: e.body,
            state: e.state.into(),
            blogger_id: e.blogger_id,
            create_at: e.create_at,
            update_at: e.update_at,
        })
        .map_err(Error::from)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, Error> {
        Entity::delete_by_id(id)
            .exec(self.conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(Error::from)
    }

    async fn update(&self, e: Blog) -> Result<Option<Blog>, Error> {
        let exist = Entity::find_by_id(e.id).one(self.conn).await;
        match exist {
            Ok(op) => match op {
                None => Ok(None),
                Some(found) => {
                    let mut found: ActiveModel = found.into();
                    found.title = Set(e.title);
                    found.body = Set(e.body);
                    found
                        .update(self.conn)
                        .await
                        .map(|updated| {
                            Some(Blog {
                                id: updated.id,
                                title: updated.title,
                                body: updated.body,
                                state: updated.state.into(),
                                blogger_id: updated.blogger_id,
                                create_at: updated.create_at,
                                update_at: updated.update_at,
                            })
                        })
                        .map_err(Error::from)
                }
            },
            Err(err) => Err(Error::from(err)),
        }
    }
}
