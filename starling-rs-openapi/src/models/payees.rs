/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Payees : The payees of an account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Payees {
    #[serde(
        rename = "payees",
        skip_serializing_if = "Option::is_none"
    )]
    pub payees: Option<Vec<crate::models::Payee>>,
}

impl Payees {
    /// The payees of an account holder
    pub fn new() -> Payees {
        Payees { payees: None }
    }
}