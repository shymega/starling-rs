/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ConfirmImageUploadRequest : Request to confirm an upload of an image of a document

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfirmImageUploadRequest {
    /// The image type of the document image being uploaded
    #[serde(rename = "idDocumentImageType")]
    pub id_document_image_type: IdDocumentImageType,
    /// The type of identity document
    #[serde(rename = "idDocumentType")]
    pub id_document_type: IdDocumentType,
    /// The filename of the identity document image that was uploaded
    #[serde(rename = "filename")]
    pub filename: String,
}

impl ConfirmImageUploadRequest {
    /// Request to confirm an upload of an image of a document
    pub fn new(
        id_document_image_type: IdDocumentImageType,
        id_document_type: IdDocumentType,
        filename: String,
    ) -> ConfirmImageUploadRequest {
        ConfirmImageUploadRequest {
            id_document_image_type,
            id_document_type,
            filename,
        }
    }
}

/// The image type of the document image being uploaded
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
pub enum IdDocumentImageType {
    #[serde(rename = "ID_PHOTO_FRONT")]
    IDPHOTOFRONT,
    #[serde(rename = "ID_PHOTO_BACK")]
    IDPHOTOBACK,
    #[serde(rename = "VIDEO")]
    VIDEO,
    #[serde(rename = "PDF_DOCUMENT")]
    PDFDOCUMENT,
    #[serde(rename = "WORD_DOCUMENT")]
    WORDDOCUMENT,
    #[serde(rename = "PROOF_OF_ADDRESS")]
    PROOFOFADDRESS,
    #[serde(rename = "PROOF_OF_NAME_CHANGE")]
    PROOFOFNAMECHANGE,
}

impl Default for IdDocumentImageType {
    fn default() -> IdDocumentImageType {
        Self::IDPHOTOFRONT
    }
}
/// The type of identity document
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
pub enum IdDocumentType {
    #[serde(rename = "PASSPORT")]
    PASSPORT,
    #[serde(rename = "PROVISIONAL_DRIVING_LICENSE")]
    PROVISIONALDRIVINGLICENSE,
    #[serde(rename = "FULL_DRIVING_LICENSE")]
    FULLDRIVINGLICENSE,
    #[serde(rename = "RESIDENCE_PERMIT")]
    RESIDENCEPERMIT,
    #[serde(rename = "IDENTITY_CARD")]
    IDENTITYCARD,
    #[serde(rename = "MARRIAGE_CERTIFICATE")]
    MARRIAGECERTIFICATE,
    #[serde(rename = "DEED_POLL")]
    DEEDPOLL,
    #[serde(rename = "DIVORCE_CERTIFICATE")]
    DIVORCECERTIFICATE,
    #[serde(rename = "BANK_STATEMENT")]
    BANKSTATEMENT,
    #[serde(rename = "UTILITY_BILL")]
    UTILITYBILL,
    #[serde(rename = "GOVERNMENT_LETTER")]
    GOVERNMENTLETTER,
    #[serde(rename = "COUNCIL_TAX")]
    COUNCILTAX,
    #[serde(rename = "FINANCIAL_STATEMENT")]
    FINANCIALSTATEMENT,
    #[serde(rename = "HOUSING_LETTER")]
    HOUSINGLETTER,
    #[serde(rename = "UNIVERSITY_LETTER")]
    UNIVERSITYLETTER,
    #[serde(rename = "NI_LETTER")]
    NILETTER,
}

impl Default for IdDocumentType {
    fn default() -> IdDocumentType {
        Self::PASSPORT
    }
}
