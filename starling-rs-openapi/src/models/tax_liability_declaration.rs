/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TaxLiabilityDeclaration : Declaration of tax liabilities

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxLiabilityDeclaration {
    /// Is the account holder liable for tax
    #[serde(rename = "taxLiabilityDeclarationAnswer")]
    pub tax_liability_declaration_answer: TaxLiabilityDeclarationAnswer,
    /// Is the account holder liable for tax in the US
    #[serde(rename = "usTaxLiabilityDeclarationAnswer")]
    pub us_tax_liability_declaration_answer:
        UsTaxLiabilityDeclarationAnswer,
    /// Countries in which account holder has tax liabilities
    #[serde(rename = "taxLiabilityDeclarationCountries")]
    pub tax_liability_declaration_countries:
        Vec<crate::models::TaxLiabilityDeclarationCountry>,
}

impl TaxLiabilityDeclaration {
    /// Declaration of tax liabilities
    pub fn new(
        tax_liability_declaration_answer: TaxLiabilityDeclarationAnswer,
        us_tax_liability_declaration_answer: UsTaxLiabilityDeclarationAnswer,
        tax_liability_declaration_countries: Vec<
            crate::models::TaxLiabilityDeclarationCountry,
        >,
    ) -> TaxLiabilityDeclaration {
        TaxLiabilityDeclaration {
            tax_liability_declaration_answer,
            us_tax_liability_declaration_answer,
            tax_liability_declaration_countries,
        }
    }
}

/// Is the account holder liable for tax
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
pub enum TaxLiabilityDeclarationAnswer {
    #[serde(rename = "YES")]
    YES,
    #[serde(rename = "NO")]
    NO,
}

impl Default for TaxLiabilityDeclarationAnswer {
    fn default() -> TaxLiabilityDeclarationAnswer {
        Self::YES
    }
}
/// Is the account holder liable for tax in the US
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
pub enum UsTaxLiabilityDeclarationAnswer {
    #[serde(rename = "YES")]
    YES,
    #[serde(rename = "NO")]
    NO,
}

impl Default for UsTaxLiabilityDeclarationAnswer {
    fn default() -> UsTaxLiabilityDeclarationAnswer {
        Self::YES
    }
}