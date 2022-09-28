/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// OutstandingActionsResponse : The status of various onboarding actions required before the onboarding process can continue

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutstandingActionsResponse {
    /// Current onboarding status of the person
    #[serde(
        rename = "status",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Status>,
    #[serde(
        rename = "personalInformationActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub personal_information_actions:
        Option<Box<crate::models::PersonalInformationActions>>,
    #[serde(
        rename = "identityDocumentActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_document_actions:
        Option<Box<crate::models::IdentityDocumentActions>>,
    #[serde(
        rename = "identityVideoActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_video_actions:
        Option<Box<crate::models::IdentityVideoActions>>,
    #[serde(
        rename = "incomeAndEmploymentActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub income_and_employment_actions:
        Option<Box<crate::models::IncomeAndEmploymentActions>>,
    #[serde(
        rename = "termsAcceptanceActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_acceptance_actions:
        Option<Box<crate::models::TermsAcceptanceActions>>,
    #[serde(
        rename = "proofOfAddressActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub proof_of_address_actions:
        Option<Box<crate::models::ProofOfAddressActions>>,
    #[serde(
        rename = "taxDeclarationActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_declaration_actions:
        Option<Box<crate::models::TaxDeclarationActions>>,
}

impl OutstandingActionsResponse {
    /// The status of various onboarding actions required before the onboarding process can continue
    pub fn new() -> OutstandingActionsResponse {
        OutstandingActionsResponse {
            status: None,
            personal_information_actions: None,
            identity_document_actions: None,
            identity_video_actions: None,
            income_and_employment_actions: None,
            terms_acceptance_actions: None,
            proof_of_address_actions: None,
            tax_declaration_actions: None,
        }
    }
}

/// Current onboarding status of the person
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
pub enum Status {
    #[serde(rename = "MORE_INFORMATION_REQUIRED")]
    MOREINFORMATIONREQUIRED,
    #[serde(rename = "READY_FOR_REVIEW")]
    READYFORREVIEW,
    #[serde(rename = "IN_REVIEW")]
    INREVIEW,
    #[serde(rename = "ACCEPTED")]
    ACCEPTED,
    #[serde(rename = "REJECTED")]
    REJECTED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
}

impl Default for Status {
    fn default() -> Status {
        Self::MOREINFORMATIONREQUIRED
    }
}
