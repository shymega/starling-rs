/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountIdentifier : Bank and account identifiers for a particular identifier type

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountIdentifier {
    /// The type of account identifier
    #[serde(
        rename = "identifierType",
        skip_serializing_if = "Option::is_none"
    )]
    pub identifier_type: Option<IdentifierType>,
    /// Identifier to uniquely identify the bank, e.g. a sort code or BIC
    #[serde(
        rename = "bankIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub bank_identifier: Option<String>,
    /// Identifier to uniquely identify the account at the bank, e.g. an account number or IBAN
    #[serde(
        rename = "accountIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_identifier: Option<String>,
}

impl AccountIdentifier {
    /// Bank and account identifiers for a particular identifier type
    pub fn new() -> AccountIdentifier {
        AccountIdentifier {
            identifier_type: None,
            bank_identifier: None,
            account_identifier: None,
        }
    }
}

/// The type of account identifier
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
pub enum IdentifierType {
    #[serde(rename = "SORT_CODE")]
    SORTCODE,
    #[serde(rename = "IBAN_BIC")]
    IBANBIC,
    #[serde(rename = "ABA_ACH")]
    ABAACH,
}

impl Default for IdentifierType {
    fn default() -> IdentifierType {
        Self::SORTCODE
    }
}
