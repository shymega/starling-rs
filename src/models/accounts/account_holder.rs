//! Module for the `Account` API.

use crate::models::enums::AccountHolderType;
use uuid::Uuid;

/// Struct for the `AccountHolder` entity from the API
#[derive(Debug)]
pub struct AccountHolder {
    pub description: String,
    pub account_holder_uid: Uuid,
    pub account_holder_type: AccountHolderType,
}

impl Default for AccountHolder {
    fn default() -> AccountHolder {
        AccountHolder {
            description: String::new(),
            account_holder_uid: Uuid::new_v4(),
            account_holder_type: AccountHolderType::UNDEFINED,
        }
    }
}
