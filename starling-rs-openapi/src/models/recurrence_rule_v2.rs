/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RecurrenceRuleV2 : The schedule definition

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecurrenceRuleV2 {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "frequency")]
    pub frequency: Frequency,
    #[serde(
        rename = "interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub interval: Option<i32>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "untilDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub until_date: Option<String>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<Days>>,
}

impl RecurrenceRuleV2 {
    /// The schedule definition
    pub fn new(
        start_date: String,
        frequency: Frequency,
    ) -> RecurrenceRuleV2 {
        RecurrenceRuleV2 {
            start_date,
            frequency,
            interval: None,
            count: None,
            until_date: None,
            days: None,
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
pub enum Frequency {
    #[serde(rename = "DAILY")]
    DAILY,
    #[serde(rename = "WEEKLY")]
    WEEKLY,
    #[serde(rename = "MONTHLY")]
    MONTHLY,
    #[serde(rename = "YEARLY")]
    YEARLY,
}

impl Default for Frequency {
    fn default() -> Frequency {
        Self::DAILY
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
pub enum Days {
    #[serde(rename = "MONDAY")]
    MONDAY,
    #[serde(rename = "TUESDAY")]
    TUESDAY,
    #[serde(rename = "WEDNESDAY")]
    WEDNESDAY,
    #[serde(rename = "THURSDAY")]
    THURSDAY,
    #[serde(rename = "FRIDAY")]
    FRIDAY,
    #[serde(rename = "SATURDAY")]
    SATURDAY,
    #[serde(rename = "SUNDAY")]
    SUNDAY,
}

impl Default for Days {
    fn default() -> Days {
        Self::MONDAY
    }
}
