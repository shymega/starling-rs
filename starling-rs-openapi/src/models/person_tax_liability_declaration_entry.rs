/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PersonTaxLiabilityDeclarationEntry : Tax liability declaration details

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonTaxLiabilityDeclarationEntry {
    /// Country code in Alpha-2 format
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Tax identification number for this declaration entry
    #[serde(rename = "taxIdentificationNumber")]
    pub tax_identification_number: String,
}

impl PersonTaxLiabilityDeclarationEntry {
    /// Tax liability declaration details
    pub fn new(
        country_code: String,
        tax_identification_number: String,
    ) -> PersonTaxLiabilityDeclarationEntry {
        PersonTaxLiabilityDeclarationEntry {
            country_code,
            tax_identification_number,
        }
    }
}
