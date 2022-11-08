/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVideoActions : ID Video Document status

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityVideoActions {
    /// Video document onboarding status
    #[serde(
        rename = "videoStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_status: Option<VideoStatus>,
    /// The reasons a video was rejected
    #[serde(
        rename = "rejectionReasons",
        skip_serializing_if = "Option::is_none"
    )]
    pub rejection_reasons: Option<Vec<RejectionReasons>>,
}

impl IdentityVideoActions {
    /// ID Video Document status
    pub fn new() -> IdentityVideoActions {
        IdentityVideoActions {
            video_status: None,
            rejection_reasons: None,
        }
    }
}

/// Video document onboarding status
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
pub enum VideoStatus {
    #[serde(rename = "AWAITING_UPLOAD")]
    AWAITINGUPLOAD,
    #[serde(rename = "READY_FOR_REVIEW")]
    READYFORREVIEW,
    #[serde(rename = "NOT_REQUIRED")]
    NOTREQUIRED,
}

impl Default for VideoStatus {
    fn default() -> VideoStatus {
        Self::AWAITINGUPLOAD
    }
}
/// The reasons a video was rejected
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
pub enum RejectionReasons {
    #[serde(rename = "NOBODY_IN_VIDEO")]
    NOBODYINVIDEO,
    #[serde(rename = "FACE_NOT_CLEAR")]
    FACENOTCLEAR,
    #[serde(rename = "NOT_CLEAR_BLURRY")]
    NOTCLEARBLURRY,
    #[serde(rename = "NOT_CLEAR_TOO_DARK")]
    NOTCLEARTOODARK,
    #[serde(rename = "NOT_CLEAR_FACE_COVERED")]
    NOTCLEARFACECOVERED,
    #[serde(rename = "NOT_CLEAR_WEARING_HAT")]
    NOTCLEARWEARINGHAT,
    #[serde(rename = "ANGLE_INCORRECT")]
    ANGLEINCORRECT,
    #[serde(rename = "PHRASE_PROBLEM")]
    PHRASEPROBLEM,
    #[serde(rename = "PHRASE_NO_AUDIO")]
    PHRASENOAUDIO,
    #[serde(rename = "PHRASE_WRONG")]
    PHRASEWRONG,
    #[serde(rename = "PHRASE_INCOMPLETE")]
    PHRASEINCOMPLETE,
    #[serde(rename = "PHRASE_SAID_BY_SOMEONE_ELSE")]
    PHRASESAIDBYSOMEONEELSE,
    #[serde(rename = "ID_VIDEO_MISMATCH")]
    IDVIDEOMISMATCH,
    #[serde(rename = "NOT_SUITABLE")]
    NOTSUITABLE,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

impl Default for RejectionReasons {
    fn default() -> RejectionReasons {
        Self::NOBODYINVIDEO
    }
}