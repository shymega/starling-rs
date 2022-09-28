/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// NextPaymentDatesResponse : List of next payment dates of a standing order

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NextPaymentDatesResponse {
    #[serde(
        rename = "nextPaymentDates",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_payment_dates: Option<Vec<String>>,
}

impl NextPaymentDatesResponse {
    /// List of next payment dates of a standing order
    pub fn new() -> NextPaymentDatesResponse {
        NextPaymentDatesResponse {
            next_payment_dates: None,
        }
    }
}
