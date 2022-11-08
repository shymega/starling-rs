/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PayeeCreationRequest : Request to create a payee

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PayeeCreationRequest {
    /// Name
    #[serde(rename = "payeeName")]
    pub payee_name: String,
    /// Phone number
    #[serde(
        rename = "phoneNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<String>,
    #[serde(rename = "payeeType")]
    pub payee_type: PayeeType,
    /// First name
    #[serde(
        rename = "firstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_name: Option<String>,
    /// Middle name
    #[serde(
        rename = "middleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_name: Option<String>,
    /// Last name
    #[serde(
        rename = "lastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_name: Option<String>,
    /// Business name
    #[serde(
        rename = "businessName",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_name: Option<String>,
    /// Date of birth for this payee
    #[serde(
        rename = "dateOfBirth",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_birth: Option<String>,
    /// List of accounts for this payee
    #[serde(
        rename = "accounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub accounts:
        Option<Vec<crate::models::PayeeAccountCreationRequest>>,
}

impl PayeeCreationRequest {
    /// Request to create a payee
    pub fn new(
        payee_name: String,
        payee_type: PayeeType,
    ) -> PayeeCreationRequest {
        PayeeCreationRequest {
            payee_name,
            phone_number: None,
            payee_type,
            first_name: None,
            middle_name: None,
            last_name: None,
            business_name: None,
            date_of_birth: None,
            accounts: None,
        }
    }
}

///
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
pub enum PayeeType {
    #[serde(rename = "BUSINESS")]
    BUSINESS,
    #[serde(rename = "INDIVIDUAL")]
    INDIVIDUAL,
}

impl Default for PayeeType {
    fn default() -> PayeeType {
        Self::BUSINESS
    }
}