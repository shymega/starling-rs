/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ErrorResponse : Erroneous response wrapper

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(
        rename = "errors",
        skip_serializing_if = "Option::is_none"
    )]
    pub errors: Option<Vec<crate::models::ErrorDetail>>,
    #[serde(
        rename = "success",
        skip_serializing_if = "Option::is_none"
    )]
    pub success: Option<bool>,
}

impl ErrorResponse {
    /// Erroneous response wrapper
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            errors: None,
            success: None,
        }
    }
}
