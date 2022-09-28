/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SavingsGoalPhotoV2 : A photo associated to a savings goal

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SavingsGoalPhotoV2 {
    /// A text (base 64) encoded picture to associate with the savings goal
    #[serde(rename = "base64EncodedPhoto")]
    pub base64_encoded_photo: String,
}

impl SavingsGoalPhotoV2 {
    /// A photo associated to a savings goal
    pub fn new(base64_encoded_photo: String) -> SavingsGoalPhotoV2 {
        SavingsGoalPhotoV2 {
            base64_encoded_photo,
        }
    }
}
