use uuid::Uuid as UUID;
use crate::models::enums::AccountCurrencies;

#[derive(Debug)]
pub struct Account {
    pub description: String,
    pub account_uid: UUID,
    pub default_category: UUID,
    pub currency: AccountCurrencies,
    pub created_at: String,
}

impl Account {
    pub fn new(description: &str, account_uid: UUID, default_category: UUID, currency: AccountCurrencies, created_at: &str) -> Account {
        Account {
            description: description.to_owned(),
            account_uid: account_uid.to_owned(),
            default_category,
            currency,
            created_at: created_at.to_owned(),
        }
    }
}

impl Default for Account {
    fn default() -> Account {
        Account {
            description: String::new(),
            account_uid: UUID::new_v4(),
            default_category: UUID::new_v4(),
            currency: AccountCurrencies::GBP,
            created_at: String::new(),
        }
    }

}
