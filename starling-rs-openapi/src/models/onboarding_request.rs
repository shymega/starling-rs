/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// OnboardingRequest : Request to onboard an account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OnboardingRequest {
    /// Account holder's mobile phone number
    #[serde(rename = "mobileNumber")]
    pub mobile_number: String,
    /// Account holder's title
    #[serde(rename = "title")]
    pub title: Title,
    /// Account holder's first name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Account holder's last name
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Account holder's date of birth
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    /// The email address for the account holder
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "currentAddress")]
    pub current_address: Box<crate::models::OnboardingAddress>,
    /// Account holder's previous address history, 6 month minimum
    #[serde(
        rename = "previousAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_addresses:
        Option<Vec<crate::models::OnboardingAddress>>,
    #[serde(rename = "incomeDetails")]
    pub income_details: Box<crate::models::IncomeDetails>,
    #[serde(rename = "taxLiabilityDeclaration")]
    pub tax_liability_declaration:
        Box<crate::models::TaxLiabilityDeclaration>,
    /// Account holder's terms acceptance details
    #[serde(rename = "termsAcceptance")]
    pub terms_acceptance: Vec<crate::models::TermsAcceptance>,
}

impl OnboardingRequest {
    /// Request to onboard an account holder
    pub fn new(
        mobile_number: String,
        title: Title,
        first_name: String,
        last_name: String,
        date_of_birth: String,
        current_address: crate::models::OnboardingAddress,
        income_details: crate::models::IncomeDetails,
        tax_liability_declaration: crate::models::TaxLiabilityDeclaration,
        terms_acceptance: Vec<crate::models::TermsAcceptance>,
    ) -> OnboardingRequest {
        OnboardingRequest {
            mobile_number,
            title,
            first_name,
            last_name,
            date_of_birth,
            email: None,
            current_address: Box::new(current_address),
            previous_addresses: None,
            income_details: Box::new(income_details),
            tax_liability_declaration: Box::new(
                tax_liability_declaration,
            ),
            terms_acceptance,
        }
    }
}

/// Account holder's title
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
pub enum Title {
    #[serde(rename = "MRS, MISS, MS, LADY, MR, SIR")]
    MRSMISSMSLADYMRSIR,
}

impl Default for Title {
    fn default() -> Title {
        Self::MRSMISSMSLADYMRSIR
    }
}