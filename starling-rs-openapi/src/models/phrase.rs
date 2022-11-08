/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Phrase : Identity verification phrase for onboarding a new customer

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Phrase {
    /// The phrase for identity verification
    #[serde(
        rename = "phrase",
        skip_serializing_if = "Option::is_none"
    )]
    pub phrase: Option<String>,
    /// The date and time that the phrase was generated
    #[serde(
        rename = "issuedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub issued_at: Option<String>,
    /// The phrase UID
    #[serde(
        rename = "phraseUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub phrase_uid: Option<String>,
}

impl Phrase {
    /// Identity verification phrase for onboarding a new customer
    pub fn new() -> Phrase {
        Phrase {
            phrase: None,
            issued_at: None,
            phrase_uid: None,
        }
    }
}