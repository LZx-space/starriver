use uuid::Uuid;

pub struct ArticleUpdate {
    pub title: String,
    pub content: String,
    pub attachment_ids: Vec<Uuid>,
    pub published: bool,
}
