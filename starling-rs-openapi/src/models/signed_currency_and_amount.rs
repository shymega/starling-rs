/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignedCurrencyAndAmount {
    /// ISO-4217 3 character currency code
    #[serde(rename = "currency")]
    pub currency: String,
    /// Amount in the minor units of the given currency; eg pence in GBP, cents in EUR
    #[serde(rename = "minorUnits")]
    pub minor_units: i64,
}

impl SignedCurrencyAndAmount {
    pub fn new(
        currency: String,
        minor_units: i64,
    ) -> SignedCurrencyAndAmount {
        SignedCurrencyAndAmount {
            currency,
            minor_units,
        }
    }
}