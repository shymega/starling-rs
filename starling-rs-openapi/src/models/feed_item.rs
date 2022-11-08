/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedItem : An item from the account holders's transaction feed

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedItem {
    /// Unique identifier for this item
    #[serde(
        rename = "feedItemUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub feed_item_uid: Option<String>,
    /// The category on which the transaction happened
    #[serde(
        rename = "categoryUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub category_uid: Option<String>,
    #[serde(
        rename = "amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Box<crate::models::CurrencyAndAmount>>,
    #[serde(
        rename = "sourceAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_amount: Option<Box<crate::models::CurrencyAndAmount>>,
    /// Was this an inbound or outbound transaction
    #[serde(
        rename = "direction",
        skip_serializing_if = "Option::is_none"
    )]
    pub direction: Option<Direction>,
    /// The time the transaction was last updated at
    #[serde(
        rename = "updatedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
    /// The time of the transaction
    #[serde(
        rename = "transactionTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<String>,
    /// The time the transaction settled
    #[serde(
        rename = "settlementTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub settlement_time: Option<String>,
    #[serde(
        rename = "retryAllocationUntilTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_allocation_until_time: Option<String>,
    /// The source of a transaction
    #[serde(
        rename = "source",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<Source>,
    /// The source subtype of the transaction
    #[serde(
        rename = "sourceSubType",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_sub_type: Option<SourceSubType>,
    /// The status of a transaction
    #[serde(
        rename = "status",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Status>,
    /// The application user that made the transaction
    #[serde(
        rename = "transactingApplicationUserUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub transacting_application_user_uid: Option<String>,
    /// The type of counter party for a transaction
    #[serde(
        rename = "counterPartyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_type: Option<CounterPartyType>,
    /// The unique identifier for the counter party. eg for MERCHANT this will be the merchant uid, for PAYEE this will be the payee uid
    #[serde(
        rename = "counterPartyUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_uid: Option<String>,
    /// The name of the counter party
    #[serde(
        rename = "counterPartyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_name: Option<String>,
    /// An identifier for the counter party sub entity. eg for MERCHANT this will be the merchant location uid, for PAYEE this will be the payee account uid
    #[serde(
        rename = "counterPartySubEntityUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_sub_entity_uid: Option<String>,
    /// A name for the counter party sub entity, for PAYEE this will be the name set for the payee account
    #[serde(
        rename = "counterPartySubEntityName",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_sub_entity_name: Option<String>,
    /// An external identifier for the sub entity, for PAYEE this will be the sort code of the account
    #[serde(
        rename = "counterPartySubEntityIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_sub_entity_identifier: Option<String>,
    /// An external sub identifier for the sub entity, for PAYEE this will be the account number of the account
    #[serde(
        rename = "counterPartySubEntitySubIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_party_sub_entity_sub_identifier: Option<String>,
    #[serde(
        rename = "exchangeRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_rate: Option<f32>,
    #[serde(
        rename = "totalFees",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_fees: Option<f32>,
    #[serde(
        rename = "totalFeeAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_fee_amount: Option<Box<crate::models::CurrencyAndAmount>>,
    /// The reference for the transaction
    #[serde(
        rename = "reference",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference: Option<String>,
    /// The country in which the transaction took place. ISO 3166-1 alpha-2
    #[serde(
        rename = "country",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<Country>,
    /// The category of a transaction
    #[serde(
        rename = "spendingCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub spending_category: Option<SpendingCategory>,
    /// The user-provided transaction note
    #[serde(
        rename = "userNote",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_note: Option<String>,
    #[serde(
        rename = "roundUp",
        skip_serializing_if = "Option::is_none"
    )]
    pub round_up: Option<Box<crate::models::AssociatedFeedRoundUp>>,
    /// Attachment present
    #[serde(
        rename = "hasAttachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_attachment: Option<bool>,
    /// Receipt present
    #[serde(
        rename = "hasReceipt",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_receipt: Option<bool>,
    #[serde(
        rename = "batchPaymentDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub batch_payment_details:
        Option<Box<crate::models::BatchPaymentDetails>>,
}

impl FeedItem {
    /// An item from the account holders's transaction feed
    pub fn new() -> FeedItem {
        FeedItem {
            feed_item_uid: None,
            category_uid: None,
            amount: None,
            source_amount: None,
            direction: None,
            updated_at: None,
            transaction_time: None,
            settlement_time: None,
            retry_allocation_until_time: None,
            source: None,
            source_sub_type: None,
            status: None,
            transacting_application_user_uid: None,
            counter_party_type: None,
            counter_party_uid: None,
            counter_party_name: None,
            counter_party_sub_entity_uid: None,
            counter_party_sub_entity_name: None,
            counter_party_sub_entity_identifier: None,
            counter_party_sub_entity_sub_identifier: None,
            exchange_rate: None,
            total_fees: None,
            total_fee_amount: None,
            reference: None,
            country: None,
            spending_category: None,
            user_note: None,
            round_up: None,
            has_attachment: None,
            has_receipt: None,
            batch_payment_details: None,
        }
    }
}

/// Was this an inbound or outbound transaction
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
pub enum Direction {
    #[serde(rename = "IN")]
    _IN,
    #[serde(rename = "OUT")]
    OUT,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::_IN
    }
}
/// The source of a transaction
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
pub enum Source {
    #[serde(rename = "BRITISH_BUSINESS_BANK_FEES")]
    BRITISHBUSINESSBANKFEES,
    #[serde(rename = "CASH_DEPOSIT")]
    CASHDEPOSIT,
    #[serde(rename = "CASH_DEPOSIT_CHARGE")]
    CASHDEPOSITCHARGE,
    #[serde(rename = "CASH_WITHDRAWAL")]
    CASHWITHDRAWAL,
    #[serde(rename = "CASH_WITHDRAWAL_CHARGE")]
    CASHWITHDRAWALCHARGE,
    #[serde(rename = "CHAPS")]
    CHAPS,
    #[serde(rename = "CHEQUE")]
    CHEQUE,
    #[serde(rename = "CICS_CHEQUE")]
    CICSCHEQUE,
    #[serde(rename = "CURRENCY_CLOUD")]
    CURRENCYCLOUD,
    #[serde(rename = "DIRECT_CREDIT")]
    DIRECTCREDIT,
    #[serde(rename = "DIRECT_DEBIT")]
    DIRECTDEBIT,
    #[serde(rename = "DIRECT_DEBIT_DISPUTE")]
    DIRECTDEBITDISPUTE,
    #[serde(rename = "INTERNAL_TRANSFER")]
    INTERNALTRANSFER,
    #[serde(rename = "MASTER_CARD")]
    MASTERCARD,
    #[serde(rename = "MASTERCARD_MONEYSEND")]
    MASTERCARDMONEYSEND,
    #[serde(rename = "MASTERCARD_CHARGEBACK")]
    MASTERCARDCHARGEBACK,
    #[serde(rename = "FASTER_PAYMENTS_IN")]
    FASTERPAYMENTSIN,
    #[serde(rename = "FASTER_PAYMENTS_OUT")]
    FASTERPAYMENTSOUT,
    #[serde(rename = "FASTER_PAYMENTS_REVERSAL")]
    FASTERPAYMENTSREVERSAL,
    #[serde(rename = "STRIPE_FUNDING")]
    STRIPEFUNDING,
    #[serde(rename = "INTEREST_PAYMENT")]
    INTERESTPAYMENT,
    #[serde(rename = "NOSTRO_DEPOSIT")]
    NOSTRODEPOSIT,
    #[serde(rename = "OVERDRAFT")]
    OVERDRAFT,
    #[serde(rename = "OVERDRAFT_INTEREST_WAIVED")]
    OVERDRAFTINTERESTWAIVED,
    #[serde(rename = "FASTER_PAYMENTS_REFUND")]
    FASTERPAYMENTSREFUND,
    #[serde(rename = "STARLING_PAY_STRIPE")]
    STARLINGPAYSTRIPE,
    #[serde(rename = "ON_US_PAY_ME")]
    ONUSPAYME,
    #[serde(rename = "LOAN_PRINCIPAL_PAYMENT")]
    LOANPRINCIPALPAYMENT,
    #[serde(rename = "LOAN_REPAYMENT")]
    LOANREPAYMENT,
    #[serde(rename = "LOAN_OVERPAYMENT")]
    LOANOVERPAYMENT,
    #[serde(rename = "LOAN_LATE_PAYMENT")]
    LOANLATEPAYMENT,
    #[serde(rename = "LOAN_FEE_PAYMENT")]
    LOANFEEPAYMENT,
    #[serde(rename = "LOAN_INTEREST_CHARGE")]
    LOANINTERESTCHARGE,
    #[serde(rename = "SEPA_CREDIT_TRANSFER")]
    SEPACREDITTRANSFER,
    #[serde(rename = "SEPA_DIRECT_DEBIT")]
    SEPADIRECTDEBIT,
    #[serde(rename = "TARGET2_CUSTOMER_PAYMENT")]
    TARGET2CUSTOMERPAYMENT,
    #[serde(rename = "FX_TRANSFER")]
    FXTRANSFER,
    #[serde(rename = "ISS_PAYMENT")]
    ISSPAYMENT,
    #[serde(rename = "STARLING_PAYMENT")]
    STARLINGPAYMENT,
    #[serde(rename = "SUBSCRIPTION_CHARGE")]
    SUBSCRIPTIONCHARGE,
    #[serde(rename = "OVERDRAFT_FEE")]
    OVERDRAFTFEE,
    #[serde(rename = "WITHHELD_TAX")]
    WITHHELDTAX,
    #[serde(rename = "ERRORS_AND_OMISSIONS")]
    ERRORSANDOMISSIONS,
}

impl Default for Source {
    fn default() -> Source {
        Self::BRITISHBUSINESSBANKFEES
    }
}
/// The source subtype of the transaction
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
pub enum SourceSubType {
    #[serde(rename = "CONTACTLESS")]
    CONTACTLESS,
    #[serde(rename = "MAGNETIC_STRIP")]
    MAGNETICSTRIP,
    #[serde(rename = "MANUAL_KEY_ENTRY")]
    MANUALKEYENTRY,
    #[serde(rename = "CHIP_AND_PIN")]
    CHIPANDPIN,
    #[serde(rename = "ONLINE")]
    ONLINE,
    #[serde(rename = "ATM")]
    ATM,
    #[serde(rename = "CREDIT_AUTH")]
    CREDITAUTH,
    #[serde(rename = "APPLE_PAY")]
    APPLEPAY,
    #[serde(rename = "ANDROID_PAY")]
    ANDROIDPAY,
    #[serde(rename = "FITBIT_PAY")]
    FITBITPAY,
    #[serde(rename = "GARMIN_PAY")]
    GARMINPAY,
    #[serde(rename = "SAMSUNG_PAY")]
    SAMSUNGPAY,
    #[serde(rename = "OTHER_WALLET")]
    OTHERWALLET,
    #[serde(rename = "CARD_SUBSCRIPTION")]
    CARDSUBSCRIPTION,
    #[serde(rename = "NOT_APPLICABLE")]
    NOTAPPLICABLE,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "DEPOSIT")]
    DEPOSIT,
    #[serde(rename = "OVERDRAFT")]
    OVERDRAFT,
    #[serde(rename = "SETTLE_UP")]
    SETTLEUP,
    #[serde(rename = "NEARBY")]
    NEARBY,
    #[serde(rename = "TRANSFER_SAME_CURRENCY")]
    TRANSFERSAMECURRENCY,
}

impl Default for SourceSubType {
    fn default() -> SourceSubType {
        Self::CONTACTLESS
    }
}
/// The status of a transaction
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
pub enum Status {
    #[serde(rename = "UPCOMING")]
    UPCOMING,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "REVERSED")]
    REVERSED,
    #[serde(rename = "SETTLED")]
    SETTLED,
    #[serde(rename = "DECLINED")]
    DECLINED,
    #[serde(rename = "REFUNDED")]
    REFUNDED,
    #[serde(rename = "RETRYING")]
    RETRYING,
    #[serde(rename = "ACCOUNT_CHECK")]
    ACCOUNTCHECK,
}

impl Default for Status {
    fn default() -> Status {
        Self::UPCOMING
    }
}
/// The type of counter party for a transaction
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
pub enum CounterPartyType {
    #[serde(rename = "CATEGORY")]
    CATEGORY,
    #[serde(rename = "CHEQUE")]
    CHEQUE,
    #[serde(rename = "CUSTOMER")]
    CUSTOMER,
    #[serde(rename = "PAYEE")]
    PAYEE,
    #[serde(rename = "MERCHANT")]
    MERCHANT,
    #[serde(rename = "SENDER")]
    SENDER,
    #[serde(rename = "STARLING")]
    STARLING,
    #[serde(rename = "LOAN")]
    LOAN,
}

impl Default for CounterPartyType {
    fn default() -> CounterPartyType {
        Self::CATEGORY
    }
}
/// The country in which the transaction took place. ISO 3166-1 alpha-2
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
pub enum Country {
    #[serde(rename = "UNDEFINED")]
    UNDEFINED,
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "AD")]
    AD,
    #[serde(rename = "AE")]
    AE,
    #[serde(rename = "AF")]
    AF,
    #[serde(rename = "AG")]
    AG,
    #[serde(rename = "AI")]
    AI,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "AN")]
    AN,
    #[serde(rename = "AO")]
    AO,
    #[serde(rename = "AQ")]
    AQ,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "AS")]
    _AS,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "AW")]
    AW,
    #[serde(rename = "AX")]
    AX,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "BB")]
    BB,
    #[serde(rename = "BD")]
    BD,
    #[serde(rename = "BE")]
    BE,
    #[serde(rename = "BF")]
    BF,
    #[serde(rename = "BG")]
    BG,
    #[serde(rename = "BH")]
    BH,
    #[serde(rename = "BI")]
    BI,
    #[serde(rename = "BJ")]
    BJ,
    #[serde(rename = "BL")]
    BL,
    #[serde(rename = "BM")]
    BM,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "BQ")]
    BQ,
    #[serde(rename = "BR")]
    BR,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "BT")]
    BT,
    #[serde(rename = "BU")]
    BU,
    #[serde(rename = "BV")]
    BV,
    #[serde(rename = "BW")]
    BW,
    #[serde(rename = "BY")]
    BY,
    #[serde(rename = "BZ")]
    BZ,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "CC")]
    CC,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CG")]
    CG,
    #[serde(rename = "CH")]
    CH,
    #[serde(rename = "CI")]
    CI,
    #[serde(rename = "CK")]
    CK,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "CN")]
    CN,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "CP")]
    CP,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "CS")]
    CS,
    #[serde(rename = "CU")]
    CU,
    #[serde(rename = "CV")]
    CV,
    #[serde(rename = "CW")]
    CW,
    #[serde(rename = "CX")]
    CX,
    #[serde(rename = "CY")]
    CY,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "DG")]
    DG,
    #[serde(rename = "DJ")]
    DJ,
    #[serde(rename = "DK")]
    DK,
    #[serde(rename = "DM")]
    DM,
    #[serde(rename = "DO")]
    _DO,
    #[serde(rename = "DZ")]
    DZ,
    #[serde(rename = "EA")]
    EA,
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "EE")]
    EE,
    #[serde(rename = "EG")]
    EG,
    #[serde(rename = "EH")]
    EH,
    #[serde(rename = "ER")]
    ER,
    #[serde(rename = "ES")]
    ES,
    #[serde(rename = "ET")]
    ET,
    #[serde(rename = "EU")]
    EU,
    #[serde(rename = "EZ")]
    EZ,
    #[serde(rename = "FI")]
    FI,
    #[serde(rename = "FJ")]
    FJ,
    #[serde(rename = "FK")]
    FK,
    #[serde(rename = "FM")]
    FM,
    #[serde(rename = "FO")]
    FO,
    #[serde(rename = "FR")]
    FR,
    #[serde(rename = "FX")]
    FX,
    #[serde(rename = "GA")]
    GA,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GD")]
    GD,
    #[serde(rename = "GE")]
    GE,
    #[serde(rename = "GF")]
    GF,
    #[serde(rename = "GG")]
    GG,
    #[serde(rename = "GH")]
    GH,
    #[serde(rename = "GI")]
    GI,
    #[serde(rename = "GL")]
    GL,
    #[serde(rename = "GM")]
    GM,
    #[serde(rename = "GN")]
    GN,
    #[serde(rename = "GP")]
    GP,
    #[serde(rename = "GQ")]
    GQ,
    #[serde(rename = "GR")]
    GR,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "GT")]
    GT,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "GW")]
    GW,
    #[serde(rename = "GY")]
    GY,
    #[serde(rename = "HK")]
    HK,
    #[serde(rename = "HM")]
    HM,
    #[serde(rename = "HN")]
    HN,
    #[serde(rename = "HR")]
    HR,
    #[serde(rename = "HT")]
    HT,
    #[serde(rename = "HU")]
    HU,
    #[serde(rename = "IC")]
    IC,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "IE")]
    IE,
    #[serde(rename = "IL")]
    IL,
    #[serde(rename = "IM")]
    IM,
    #[serde(rename = "IN")]
    _IN,
    #[serde(rename = "IO")]
    IO,
    #[serde(rename = "IQ")]
    IQ,
    #[serde(rename = "IR")]
    IR,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "JE")]
    JE,
    #[serde(rename = "JM")]
    JM,
    #[serde(rename = "JO")]
    JO,
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "KE")]
    KE,
    #[serde(rename = "KG")]
    KG,
    #[serde(rename = "KH")]
    KH,
    #[serde(rename = "KI")]
    KI,
    #[serde(rename = "KM")]
    KM,
    #[serde(rename = "KN")]
    KN,
    #[serde(rename = "KP")]
    KP,
    #[serde(rename = "KR")]
    KR,
    #[serde(rename = "KW")]
    KW,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "KZ")]
    KZ,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "LC")]
    LC,
    #[serde(rename = "LI")]
    LI,
    #[serde(rename = "LK")]
    LK,
    #[serde(rename = "LR")]
    LR,
    #[serde(rename = "LS")]
    LS,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "LU")]
    LU,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "LY")]
    LY,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "MC")]
    MC,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "MF")]
    MF,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "MH")]
    MH,
    #[serde(rename = "MK")]
    MK,
    #[serde(rename = "ML")]
    ML,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "MO")]
    MO,
    #[serde(rename = "MP")]
    MP,
    #[serde(rename = "MQ")]
    MQ,
    #[serde(rename = "MR")]
    MR,
    #[serde(rename = "MS")]
    MS,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MU")]
    MU,
    #[serde(rename = "MV")]
    MV,
    #[serde(rename = "MW")]
    MW,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "MY")]
    MY,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "NA")]
    NA,
    #[serde(rename = "NC")]
    NC,
    #[serde(rename = "NE")]
    NE,
    #[serde(rename = "NF")]
    NF,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NI")]
    NI,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "NO")]
    NO,
    #[serde(rename = "NP")]
    NP,
    #[serde(rename = "NR")]
    NR,
    #[serde(rename = "NT")]
    NT,
    #[serde(rename = "NU")]
    NU,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "OM")]
    OM,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "PF")]
    PF,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "PH")]
    PH,
    #[serde(rename = "PK")]
    PK,
    #[serde(rename = "PL")]
    PL,
    #[serde(rename = "PM")]
    PM,
    #[serde(rename = "PN")]
    PN,
    #[serde(rename = "PR")]
    PR,
    #[serde(rename = "PS")]
    PS,
    #[serde(rename = "PT")]
    PT,
    #[serde(rename = "PW")]
    PW,
    #[serde(rename = "PY")]
    PY,
    #[serde(rename = "QA")]
    QA,
    #[serde(rename = "RE")]
    RE,
    #[serde(rename = "RO")]
    RO,
    #[serde(rename = "RS")]
    RS,
    #[serde(rename = "RU")]
    RU,
    #[serde(rename = "RW")]
    RW,
    #[serde(rename = "SA")]
    SA,
    #[serde(rename = "SB")]
    SB,
    #[serde(rename = "SC")]
    SC,
    #[serde(rename = "SD")]
    SD,
    #[serde(rename = "SE")]
    SE,
    #[serde(rename = "SF")]
    SF,
    #[serde(rename = "SG")]
    SG,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SJ")]
    SJ,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "SL")]
    SL,
    #[serde(rename = "SM")]
    SM,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SO")]
    SO,
    #[serde(rename = "SR")]
    SR,
    #[serde(rename = "SS")]
    SS,
    #[serde(rename = "ST")]
    ST,
    #[serde(rename = "SU")]
    SU,
    #[serde(rename = "SV")]
    SV,
    #[serde(rename = "SX")]
    SX,
    #[serde(rename = "SY")]
    SY,
    #[serde(rename = "SZ")]
    SZ,
    #[serde(rename = "TA")]
    TA,
    #[serde(rename = "TC")]
    TC,
    #[serde(rename = "TD")]
    TD,
    #[serde(rename = "TF")]
    TF,
    #[serde(rename = "TG")]
    TG,
    #[serde(rename = "TH")]
    TH,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "TK")]
    TK,
    #[serde(rename = "TL")]
    TL,
    #[serde(rename = "TM")]
    TM,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "TO")]
    TO,
    #[serde(rename = "TP")]
    TP,
    #[serde(rename = "TR")]
    TR,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "TW")]
    TW,
    #[serde(rename = "TZ")]
    TZ,
    #[serde(rename = "UA")]
    UA,
    #[serde(rename = "UG")]
    UG,
    #[serde(rename = "UK")]
    UK,
    #[serde(rename = "UM")]
    UM,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "UY")]
    UY,
    #[serde(rename = "UZ")]
    UZ,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "VC")]
    VC,
    #[serde(rename = "VE")]
    VE,
    #[serde(rename = "VG")]
    VG,
    #[serde(rename = "VI")]
    VI,
    #[serde(rename = "VN")]
    VN,
    #[serde(rename = "VU")]
    VU,
    #[serde(rename = "WF")]
    WF,
    #[serde(rename = "WS")]
    WS,
    #[serde(rename = "XI")]
    XI,
    #[serde(rename = "XU")]
    XU,
    #[serde(rename = "XK")]
    XK,
    #[serde(rename = "YE")]
    YE,
    #[serde(rename = "YT")]
    YT,
    #[serde(rename = "YU")]
    YU,
    #[serde(rename = "ZA")]
    ZA,
    #[serde(rename = "ZM")]
    ZM,
    #[serde(rename = "ZR")]
    ZR,
    #[serde(rename = "ZW")]
    ZW,
}

impl Default for Country {
    fn default() -> Country {
        Self::UNDEFINED
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