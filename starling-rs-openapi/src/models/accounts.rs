/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Accounts : The accounts of an account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Accounts {
    #[serde(
        rename = "accounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub accounts: Option<Vec<crate::models::AccountV2>>,
}

impl Accounts {
    /// The accounts of an account holder
    pub fn new() -> Accounts {
        Accounts { accounts: None }
    }
}
