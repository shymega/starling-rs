/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RecurringCardPayment : List of recurring card payments

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecurringCardPayment {
    /// Unique identifier for the recurring card payment
    #[serde(
        rename = "recurringPaymentUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub recurring_payment_uid: Option<String>,
    /// Unique identifier for the account
    #[serde(
        rename = "accountUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_uid: Option<String>,
    /// Unique identifier for the counterparty
    #[serde(
        rename = "counterPartyUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_uid: Option<String>,
    /// Counterparty name
    #[serde(
        rename = "counterPartyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_name: Option<String>,
    /// Recurring payment status
    #[serde(
        rename = "status",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Status>,
    /// Unique identifier for the latest feed item
    #[serde(
        rename = "latestFeedItemUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_feed_item_uid: Option<String>,
    /// Latest payment date
    #[serde(
        rename = "latestPaymentDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_payment_date: Option<String>,
    #[serde(
        rename = "latestPaymentAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_payment_amount:
        Option<Box<crate::models::CurrencyAndAmount>>,
    /// Unique identifier for the feed item
    #[serde(
        rename = "mostRecentFeedItem",
        skip_serializing_if = "Option::is_none"
    )]
    pub most_recent_feed_item: Option<String>,
    /// Most recent payment date
    #[serde(
        rename = "mostRecentPaymentDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub most_recent_payment_date: Option<String>,
    #[serde(
        rename = "mostRecentPaymentAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub most_recent_payment_amount: Option<Box<crate::models::Money>>,
}

impl RecurringCardPayment {
    /// List of recurring card payments
    pub fn new() -> RecurringCardPayment {
        RecurringCardPayment {
            recurring_payment_uid: None,
            account_uid: None,
            counter_party_uid: None,
            counter_party_name: None,
            status: None,
            latest_feed_item_uid: None,
            latest_payment_date: None,
            latest_payment_amount: None,
            most_recent_feed_item: None,
            most_recent_payment_date: None,
            most_recent_payment_amount: None,
        }
    }
}

/// Recurring payment status
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "MARKED_HIDDEN")]
    MARKEDHIDDEN,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
}

impl Default for Status {
    fn default() -> Status {
        Self::ACTIVE
    }
}
