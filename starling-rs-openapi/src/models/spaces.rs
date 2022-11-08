/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Spaces : A list containing all spaces for an account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Spaces {
    #[serde(rename = "savingsGoals")]
    pub savings_goals: Vec<crate::models::SavingsGoalOrdered>,
    #[serde(rename = "spendingSpaces")]
    pub spending_spaces: Vec<crate::models::SpendingSpace>,
}

impl Spaces {
    /// A list containing all spaces for an account holder
    pub fn new(
        savings_goals: Vec<crate::models::SavingsGoalOrdered>,
        spending_spaces: Vec<crate::models::SpendingSpace>,
    ) -> Spaces {
        Spaces {
            savings_goals,
            spending_spaces,
        }
    }
}