use uuid::Uuid as UUID;

#[derive(Debug, Default)]
pub struct Account {
    pub description: String,
    pub account_uid: UUID,
    pub default_category: UUID,
    pub currency: String,
    pub created_at: String,
}

impl Account {
    #[allow(clippy::clippy::must_use_candidate)] // need to look into this lint, allowing for now.
    pub fn new(
        description: &str,
        account_uid: UUID,
        default_category: UUID,
        currency: &str,
        created_at: &str,
    ) -> Account {
        Account {
            description: description.to_owned(),
            account_uid,
            default_category,
            currency: currency.to_owned(),
            created_at: created_at.to_owned(),
        }
    }
}
