use crate::models::enums::account_enums::AccountHolderType;
use uuid::Uuid as UUID;

pub struct AccountHolder {
    pub description: String,
    pub account_holder_uid: UUID,
    pub account_holder_type: AccountHolderType,
}
