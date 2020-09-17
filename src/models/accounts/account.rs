use uuid::Uuid as UUID;

pub struct Account {
    pub description: String,
    pub account_uid: UUID,
    pub default_category: UUID,
    pub currency: String,
    pub created_at: String,
}