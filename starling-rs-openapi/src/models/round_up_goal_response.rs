/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoundUpGoalResponse {
    #[serde(
        rename = "active",
        skip_serializing_if = "Option::is_none"
    )]
    pub active: Option<bool>,
    #[serde(
        rename = "roundUpGoal",
        skip_serializing_if = "Option::is_none"
    )]
    pub round_up_goal: Option<Box<crate::models::RoundUpGoalDetails>>,
    #[serde(
        rename = "roundUpGoalDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub round_up_goal_details:
        Option<Box<crate::models::RoundUpGoalDetails>>,
}

impl RoundUpGoalResponse {
    pub fn new() -> RoundUpGoalResponse {
        RoundUpGoalResponse {
            active: None,
            round_up_goal: None,
            round_up_goal_details: None,
        }
    }
}