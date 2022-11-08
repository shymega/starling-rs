/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountHolderCreationRequest : Request to create a account holder and an account for an onboarded individual

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountHolderCreationRequest {
    /// The currency for the underlying account
    #[serde(rename = "accountCurrency")]
    pub account_currency: String,
}

impl AccountHolderCreationRequest {
    /// Request to create a account holder and an account for an onboarded individual
    pub fn new(
        account_currency: String,
    ) -> AccountHolderCreationRequest {
        AccountHolderCreationRequest { account_currency }
    }
}