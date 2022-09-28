/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PresignedUrl : Response providing a presigned url

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PresignedUrl {
    /// Presigned media document upload URL
    #[serde(
        rename = "presignedUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub presigned_url: Option<String>,
}

impl PresignedUrl {
    /// Response providing a presigned url
    pub fn new() -> PresignedUrl {
        PresignedUrl {
            presigned_url: None,
        }
    }
}
