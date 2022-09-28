/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// BalanceV2 : Balance details

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BalanceV2 {
    #[serde(
        rename = "clearedBalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub cleared_balance:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "effectiveBalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_balance:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "pendingTransactions",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_transactions:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "acceptedOverdraft",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_overdraft:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "totalClearedBalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cleared_balance:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
    #[serde(
        rename = "totalEffectiveBalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_effective_balance:
        Option<Box<crate::models::SignedCurrencyAndAmount>>,
}

impl BalanceV2 {
    /// Balance details
    pub fn new() -> BalanceV2 {
        BalanceV2 {
            cleared_balance: None,
            effective_balance: None,
            pending_transactions: None,
            accepted_overdraft: None,
            amount: None,
            total_cleared_balance: None,
            total_effective_balance: None,
        }
    }
}
