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
pub struct FetchSourcesOfFunds200Response {
    #[serde(
        rename = "sourcesOfFunds",
        skip_serializing_if = "Option::is_none"
    )]
    pub sources_of_funds:
        Option<std::collections::HashSet<SourcesOfFunds>>,
    #[serde(
        rename = "sourceOfFunds",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_of_funds:
        Option<std::collections::HashSet<SourceOfFunds>>,
}

impl FetchSourcesOfFunds200Response {
    pub fn new() -> FetchSourcesOfFunds200Response {
        FetchSourcesOfFunds200Response {
            sources_of_funds: None,
            source_of_funds: None,
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
pub enum SourcesOfFunds {
    #[serde(rename = "BENEFITS")]
    BENEFITS,
    #[serde(rename = "FRIENDS_AND_FAMILY")]
    FRIENDSANDFAMILY,
    #[serde(rename = "INVESTMENT")]
    INVESTMENT,
    #[serde(rename = "LOANS")]
    LOANS,
    #[serde(rename = "MONTHLY_SALARY")]
    MONTHLYSALARY,
    #[serde(rename = "PENSION")]
    PENSION,
    #[serde(rename = "PROPERTY")]
    PROPERTY,
    #[serde(rename = "WEEKLY_WAGES")]
    WEEKLYWAGES,
}

impl Default for SourcesOfFunds {
    fn default() -> SourcesOfFunds {
        Self::BENEFITS
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
pub enum SourceOfFunds {
    #[serde(rename = "BENEFITS")]
    BENEFITS,
    #[serde(rename = "FRIENDS_AND_FAMILY")]
    FRIENDSANDFAMILY,
    #[serde(rename = "INVESTMENT")]
    INVESTMENT,
    #[serde(rename = "LOANS")]
    LOANS,
    #[serde(rename = "MONTHLY_SALARY")]
    MONTHLYSALARY,
    #[serde(rename = "PENSION")]
    PENSION,
    #[serde(rename = "PROPERTY")]
    PROPERTY,
    #[serde(rename = "WEEKLY_WAGES")]
    WEEKLYWAGES,
}

impl Default for SourceOfFunds {
    fn default() -> SourceOfFunds {
        Self::BENEFITS
    }
}