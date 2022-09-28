/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddressV2 : Physical address of account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressV2 {
    #[serde(rename = "line1")]
    pub line1: String,
    #[serde(rename = "line2", skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(rename = "line3", skip_serializing_if = "Option::is_none")]
    pub line3: Option<String>,
    #[serde(rename = "postTown")]
    pub post_town: String,
    #[serde(rename = "postCode")]
    pub post_code: String,
    /// Country code in ISO 3166-1 alpha-2. NB GB is the official country code for the UK. UK is not the officially assigned code, and so is not valid
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

impl AddressV2 {
    /// Physical address of account holder
    pub fn new(
        line1: String,
        post_town: String,
        post_code: String,
        country_code: String,
    ) -> AddressV2 {
        AddressV2 {
            line1,
            line2: None,
            line3: None,
            post_town,
            post_code,
            country_code,
        }
    }
}
