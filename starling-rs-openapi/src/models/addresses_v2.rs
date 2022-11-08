/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddressesV2 : Current and previous physical addresses

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressesV2 {
    #[serde(
        rename = "current",
        skip_serializing_if = "Option::is_none"
    )]
    pub current: Option<Box<crate::models::AddressV2>>,
    #[serde(
        rename = "previous",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous: Option<Vec<crate::models::AddressV2>>,
}

impl AddressesV2 {
    /// Current and previous physical addresses
    pub fn new() -> AddressesV2 {
        AddressesV2 {
            current: None,
            previous: None,
        }
    }
}