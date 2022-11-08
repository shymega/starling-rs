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
pub struct ReceiptItem {
    #[serde(
        rename = "receiptItemUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_item_uid: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(
        rename = "quantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<i32>,
    #[serde(rename = "amount")]
    pub amount: f32,
    #[serde(rename = "tax")]
    pub tax: f32,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The time the receipt item was created
    #[serde(
        rename = "creationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_time: Option<String>,
    /// Type of receipt item. It can be PURCHASE, SERVICE_FEE, GRATUITY or null
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    #[serde(
        rename = "subItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_items: Option<Vec<crate::models::ReceiptSubItem>>,
}

impl ReceiptItem {
    pub fn new(
        description: String,
        amount: f32,
        tax: f32,
    ) -> ReceiptItem {
        ReceiptItem {
            receipt_item_uid: None,
            description,
            quantity: None,
            amount,
            tax,
            url: None,
            creation_time: None,
            _type: None,
            notes: None,
            sub_items: None,
        }
    }
}

/// Type of receipt item. It can be PURCHASE, SERVICE_FEE, GRATUITY or null
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
pub enum Type {
    #[serde(rename = "PURCHASE")]
    PURCHASE,
    #[serde(rename = "SERVICE_FEE")]
    SERVICEFEE,
    #[serde(rename = "GRATUITY")]
    GRATUITY,
    #[serde(rename = "BALANCE_ADJUSTMENT")]
    BALANCEADJUSTMENT,
}

impl Default for Type {
    fn default() -> Type {
        Self::PURCHASE
    }
}