use uuid::Uuid;

pub struct BlogUpdate {
    pub title: String,
    pub content: String,
    pub attachment_ids: Vec<Uuid>,
    pub published: bool,
}
