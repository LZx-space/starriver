use uuid::Uuid;

pub struct ArticleUpdate {
    pub title: String,
    pub content: String,
    pub category_id: Uuid,
    pub attachment_ids: Vec<Uuid>,
    pub published: bool,
}
