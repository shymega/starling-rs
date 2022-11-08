/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Business : Details of a business account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Business {
    #[serde(
        rename = "companyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_name: Option<String>,
    #[serde(
        rename = "companyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<String>,
    #[serde(
        rename = "companyCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_category: Option<String>,
    #[serde(
        rename = "companySubCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_sub_category: Option<String>,
    #[serde(
        rename = "companyRegistrationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_registration_number: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl Business {
    /// Details of a business account holder
    pub fn new() -> Business {
        Business {
            company_name: None,
            company_type: None,
            company_category: None,
            company_sub_category: None,
            company_registration_number: None,
            email: None,
            phone: None,
        }
    }
}