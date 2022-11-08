/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreatePersonRequest : Request to register an individual as part of onboarding

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePersonRequest {
    /// External identifier for the request
    #[serde(rename = "externalIdentifier")]
    pub external_identifier: String,
    /// Account holder's mobile phone number
    #[serde(rename = "mobileNumber")]
    pub mobile_number: String,
    /// Account holder's mobile phone number has been verified
    #[serde(rename = "mobileNumberVerified")]
    pub mobile_number_verified: bool,
    /// Account holder's title
    #[serde(rename = "title")]
    pub title: Title,
    /// Account holder's preferred name
    #[serde(
        rename = "preferredName",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_name: Option<String>,
    /// Account holder's first name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Account holder's middle name
    #[serde(
        rename = "middleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_name: Option<String>,
    /// Account holder's last name
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Account holder's date of birth
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    /// The email address for the account holder
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "currentAddress")]
    pub current_address: Box<crate::models::AddressRequest>,
    /// Account holder's previous address history, 6 month minimum
    #[serde(
        rename = "previousAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_addresses: Option<Vec<crate::models::AddressRequest>>,
    /// Has provided address history covering at least six months
    #[serde(rename = "addressHistoryCoversMinimumTerm")]
    pub address_history_covers_minimum_term: bool,
}

impl CreatePersonRequest {
    /// Request to register an individual as part of onboarding
    pub fn new(
        external_identifier: String,
        mobile_number: String,
        mobile_number_verified: bool,
        title: Title,
        first_name: String,
        last_name: String,
        date_of_birth: String,
        email: String,
        current_address: crate::models::AddressRequest,
        address_history_covers_minimum_term: bool,
    ) -> CreatePersonRequest {
        CreatePersonRequest {
            external_identifier,
            mobile_number,
            mobile_number_verified,
            title,
            preferred_name: None,
            first_name,
            middle_name: None,
            last_name,
            date_of_birth,
            email,
            current_address: Box::new(current_address),
            previous_addresses: None,
            address_history_covers_minimum_term,
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