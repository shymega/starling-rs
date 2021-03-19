use crate::models::common::CurrencyAndAmount;
use uuid::Uuid as UUID;

type DateTimeISO = String;

#[derive(Clone, Debug, Default)]
#[allow(non_snake_case)]
pub struct Transaction {
    pub feed_item_uid: UUID,
    pub category_uid: UUID,
    pub amount: CurrencyAndAmount,
    pub sourceAmount: CurrencyAndAmount,
    pub direction: String,
    pub updatedAt: DateTimeISO,
    pub transactionTime: DateTimeISO,
    pub settlementTime: DateTimeISO,
    pub source: String,
    pub status: String,
    pub counterPartytype: String,
    pub counterPartyUid: UUID,
    pub counterPartyName: String,
    pub counterPartySubEntityUid: UUID,
    pub counterPartySubEntityName: String,
    pub counterPartySubEntityIdentifier: String,
    pub counterPartySubEntitySubIdentifier: String,
    pub reference: String,
    pub country: String,
    pub spendingCategory: String,
    pub hasAttachment: bool,
}
