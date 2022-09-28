/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// InstructLocalPaymentRequest : Request to make a local payment

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstructLocalPaymentRequest {
    /// Identifier for the payment request, generated by you. Must be unique for each payment request you create
    #[serde(rename = "externalIdentifier")]
    pub external_identifier: String,
    /// The desired payment recipient. Either paymentRecipient or destinationPayeeAccountUid must be present
    #[serde(
        rename = "destinationPayeeAccountUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_payee_account_uid: Option<String>,
    #[serde(
        rename = "paymentRecipient",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_recipient: Option<Box<crate::models::PaymentRecipient>>,
    /// The payment reference. The 35 character limit applies to SEPA payments (Euro accounts). For FPS payments (GBP accounts), the limit is 18 characters
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "amount")]
    pub amount: Box<crate::models::CurrencyAndAmount>,
    /// The category of a transaction
    #[serde(
        rename = "spendingCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub spending_category: Option<SpendingCategory>,
}

impl InstructLocalPaymentRequest {
    /// Request to make a local payment
    pub fn new(
        external_identifier: String,
        reference: String,
        amount: crate::models::CurrencyAndAmount,
    ) -> InstructLocalPaymentRequest {
        InstructLocalPaymentRequest {
            external_identifier,
            destination_payee_account_uid: None,
            payment_recipient: None,
            reference,
            amount: Box::new(amount),
            spending_category: None,
        }
    }
}

/// The category of a transaction
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
pub enum SpendingCategory {
    #[serde(rename = "BIKE")]
    BIKE,
    #[serde(rename = "BILLS_AND_SERVICES")]
    BILLSANDSERVICES,
    #[serde(rename = "BUCKET_LIST")]
    BUCKETLIST,
    #[serde(rename = "CAR")]
    CAR,
    #[serde(rename = "CASH")]
    CASH,
    #[serde(rename = "CELEBRATION")]
    CELEBRATION,
    #[serde(rename = "CHARITY")]
    CHARITY,
    #[serde(rename = "CHILDREN")]
    CHILDREN,
    #[serde(rename = "COFFEE")]
    COFFEE,
    #[serde(rename = "DEBT_REPAYMENT")]
    DEBTREPAYMENT,
    #[serde(rename = "DIY")]
    DIY,
    #[serde(rename = "DRINKS")]
    DRINKS,
    #[serde(rename = "EATING_OUT")]
    EATINGOUT,
    #[serde(rename = "EDUCATION")]
    EDUCATION,
    #[serde(rename = "EMERGENCY")]
    EMERGENCY,
    #[serde(rename = "ENTERTAINMENT")]
    ENTERTAINMENT,
    #[serde(rename = "ESSENTIAL_SPEND")]
    ESSENTIALSPEND,
    #[serde(rename = "EXPENSES")]
    EXPENSES,
    #[serde(rename = "FAMILY")]
    FAMILY,
    #[serde(rename = "FITNESS")]
    FITNESS,
    #[serde(rename = "FUEL")]
    FUEL,
    #[serde(rename = "GAMBLING")]
    GAMBLING,
    #[serde(rename = "GAMING")]
    GAMING,
    #[serde(rename = "GARDEN")]
    GARDEN,
    #[serde(rename = "GENERAL")]
    GENERAL,
    #[serde(rename = "GIFTS")]
    GIFTS,
    #[serde(rename = "GROCERIES")]
    GROCERIES,
    #[serde(rename = "HOBBY")]
    HOBBY,
    #[serde(rename = "HOLIDAYS")]
    HOLIDAYS,
    #[serde(rename = "HOME")]
    HOME,
    #[serde(rename = "IMPULSE_BUY")]
    IMPULSEBUY,
    #[serde(rename = "INCOME")]
    INCOME,
    #[serde(rename = "INSURANCE")]
    INSURANCE,
    #[serde(rename = "INVESTMENTS")]
    INVESTMENTS,
    #[serde(rename = "LIFESTYLE")]
    LIFESTYLE,
    #[serde(rename = "MAINTENANCE_AND_REPAIRS")]
    MAINTENANCEANDREPAIRS,
    #[serde(rename = "MEDICAL")]
    MEDICAL,
    #[serde(rename = "MORTGAGE")]
    MORTGAGE,
    #[serde(rename = "NON_ESSENTIAL_SPEND")]
    NONESSENTIALSPEND,
    #[serde(rename = "PAYMENTS")]
    PAYMENTS,
    #[serde(rename = "PERSONAL_TRANSFERS")]
    PERSONALTRANSFERS,
    #[serde(rename = "PETS")]
    PETS,
    #[serde(rename = "PROJECTS")]
    PROJECTS,
    #[serde(rename = "RELATIONSHIPS")]
    RELATIONSHIPS,
    #[serde(rename = "RENT")]
    RENT,
    #[serde(rename = "SAVING")]
    SAVING,
    #[serde(rename = "SHOPPING")]
    SHOPPING,
    #[serde(rename = "SUBSCRIPTIONS")]
    SUBSCRIPTIONS,
    #[serde(rename = "TAKEAWAY")]
    TAKEAWAY,
    #[serde(rename = "TAXI")]
    TAXI,
    #[serde(rename = "TRANSPORT")]
    TRANSPORT,
    #[serde(rename = "TREATS")]
    TREATS,
    #[serde(rename = "WEDDING")]
    WEDDING,
    #[serde(rename = "WELLBEING")]
    WELLBEING,
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "REVENUE")]
    REVENUE,
    #[serde(rename = "OTHER_INCOME")]
    OTHERINCOME,
    #[serde(rename = "CLIENT_REFUNDS")]
    CLIENTREFUNDS,
    #[serde(rename = "INVENTORY")]
    INVENTORY,
    #[serde(rename = "STAFF")]
    STAFF,
    #[serde(rename = "TRAVEL")]
    TRAVEL,
    #[serde(rename = "WORKPLACE")]
    WORKPLACE,
    #[serde(rename = "REPAIRS_AND_MAINTENANCE")]
    REPAIRSANDMAINTENANCE,
    #[serde(rename = "ADMIN")]
    ADMIN,
    #[serde(rename = "MARKETING")]
    MARKETING,
    #[serde(rename = "BUSINESS_ENTERTAINMENT")]
    BUSINESSENTERTAINMENT,
    #[serde(rename = "INTEREST_PAYMENTS")]
    INTERESTPAYMENTS,
    #[serde(rename = "BANK_CHARGES")]
    BANKCHARGES,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "FOOD_AND_DRINK")]
    FOODANDDRINK,
    #[serde(rename = "EQUIPMENT")]
    EQUIPMENT,
    #[serde(rename = "PROFESSIONAL_SERVICES")]
    PROFESSIONALSERVICES,
    #[serde(rename = "PHONE_AND_INTERNET")]
    PHONEANDINTERNET,
    #[serde(rename = "VEHICLES")]
    VEHICLES,
    #[serde(rename = "DIRECTORS_WAGES")]
    DIRECTORSWAGES,
    #[serde(rename = "VAT")]
    VAT,
    #[serde(rename = "CORPORATION_TAX")]
    CORPORATIONTAX,
    #[serde(rename = "SELF_ASSESSMENT_TAX")]
    SELFASSESSMENTTAX,
    #[serde(rename = "INVESTMENT_CAPITAL")]
    INVESTMENTCAPITAL,
    #[serde(rename = "TRANSFERS")]
    TRANSFERS,
    #[serde(rename = "LOAN_PRINCIPAL")]
    LOANPRINCIPAL,
    #[serde(rename = "PERSONAL")]
    PERSONAL,
    #[serde(rename = "DIVIDENDS")]
    DIVIDENDS,
}

impl Default for SpendingCategory {
    fn default() -> SpendingCategory {
        Self::BIKE
    }
}
