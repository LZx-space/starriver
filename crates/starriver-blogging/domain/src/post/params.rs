use uuid::Uuid;

pub struct PostUpdate {
    pub title: String,
    pub content: String,
    pub category_id: Uuid,
    pub attachments: Vec<Uuid>,
    pub published: bool,
}
