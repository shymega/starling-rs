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
pub struct ReceiptSubItem {
    #[serde(
        rename = "receiptSubItemUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_sub_item_uid: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(
        rename = "quantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<i32>,
    #[serde(
        rename = "amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<f32>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

impl ReceiptSubItem {
    pub fn new(description: String) -> ReceiptSubItem {
        ReceiptSubItem {
            receipt_sub_item_uid: None,
            description,
            quantity: None,
            amount: None,
            notes: None,
        }
    }
}
