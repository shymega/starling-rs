/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Payments : List of payments

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Payments {
    #[serde(
        rename = "payments",
        skip_serializing_if = "Option::is_none"
    )]
    pub payments: Option<Vec<crate::models::PayeePayment>>,
}

impl Payments {
    /// List of payments
    pub fn new() -> Payments {
        Payments { payments: None }
    }
}