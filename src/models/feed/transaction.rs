use crate::models::common::CurrencyAndAmount;
use uuid::Uuid as UUID;
use crate::models::enums::{Country, CounterPartyType};

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
    pub counterPartytype: CounterPartyType,
    pub counterPartyUid: UUID,
    pub counterPartyName: String,
    pub counterPartySubEntityUid: UUID,
    pub counterPartySubEntityName: String,
    pub counterPartySubEntityIdentifier: String,
    pub counterPartySubEntitySubIdentifier: String,
    pub reference: String,
    pub country: Country,
    pub spendingCategory: String,
    pub hasAttachment: bool,
}
