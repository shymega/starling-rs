/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ConfirmUploadRequest : Request to confirm a document has been uploaded

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfirmUploadRequest {
    /// The phrase UID
    #[serde(rename = "phraseUid")]
    pub phrase_uid: String,
    /// The filename of the video that was uploaded
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(
        rename = "contentType",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type:
        Option<Box<crate::models::ConfirmUploadRequestContentType>>,
    #[serde(rename = "mimeType")]
    pub mime_type: Box<crate::models::PresignedUploadRequestMimeType>,
}

impl ConfirmUploadRequest {
    /// Request to confirm a document has been uploaded
    pub fn new(
        phrase_uid: String,
        filename: String,
        mime_type: crate::models::PresignedUploadRequestMimeType,
    ) -> ConfirmUploadRequest {
        ConfirmUploadRequest {
            phrase_uid,
            filename,
            content_type: None,
            mime_type: Box::new(mime_type),
        }
    }
}
