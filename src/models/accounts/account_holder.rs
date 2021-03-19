//! Module for the `Account` API.

use crate::models::enums::AccountHolderType;
use uuid::Uuid;

/// Struct for the `AccountHolder` entity from the API
#[derive(Debug, Default)]
pub struct AccountHolder {
    pub description: String,
    pub account_holder_uid: Uuid,
    pub account_holder_type: AccountHolderType,
}
