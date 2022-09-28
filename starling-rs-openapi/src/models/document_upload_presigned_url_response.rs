/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentUploadPresignedUrlResponse : A temporary presigned upload URL for uploading a supporting document and a unique identifier to reference that document in later calls

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentUploadPresignedUrlResponse {
    /// URL where a document file can be uploaded. Valid for only 10 minutes.
    #[serde(
        rename = "presignedUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub presigned_url: Option<String>,
    /// Unique identifier for the name change document to be uploaded
    #[serde(
        rename = "personNameChangeDocumentUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub person_name_change_document_uid: Option<String>,
}

impl DocumentUploadPresignedUrlResponse {
    /// A temporary presigned upload URL for uploading a supporting document and a unique identifier to reference that document in later calls
    pub fn new() -> DocumentUploadPresignedUrlResponse {
        DocumentUploadPresignedUrlResponse {
            presigned_url: None,
            person_name_change_document_uid: None,
        }
    }
}
