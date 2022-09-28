/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentOrderPayment : Response object for payment order payments queries

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentOrderPayment {
    /// UID of this payment
    #[serde(rename = "paymentUid")]
    pub payment_uid: String,
    #[serde(rename = "amount")]
    pub amount: Box<crate::models::CurrencyAndAmount>,
    /// Reference of this payment
    #[serde(rename = "reference")]
    pub reference: String,
    /// The ID of the payee receiving this payment
    #[serde(rename = "payeeUid")]
    pub payee_uid: String,
    /// The account ID of the payee account receiving this payment
    #[serde(rename = "payeeAccountUid")]
    pub payee_account_uid: String,
    /// When this payment was created
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// When this payment was completed, if it completed
    #[serde(
        rename = "completedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
    /// When this payment was rejected, if it was rejected
    #[serde(
        rename = "rejectedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub rejected_at: Option<String>,
    #[serde(
        rename = "paymentStatusDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_status_details:
        Option<Box<crate::models::PaymentStatusDetails>>,
}

impl PaymentOrderPayment {
    /// Response object for payment order payments queries
    pub fn new(
        payment_uid: String,
        amount: crate::models::CurrencyAndAmount,
        reference: String,
        payee_uid: String,
        payee_account_uid: String,
        created_at: String,
    ) -> PaymentOrderPayment {
        PaymentOrderPayment {
            payment_uid,
            amount: Box::new(amount),
            reference,
            payee_uid,
            payee_account_uid,
            created_at,
            completed_at: None,
            rejected_at: None,
            payment_status_details: None,
        }
    }
}
