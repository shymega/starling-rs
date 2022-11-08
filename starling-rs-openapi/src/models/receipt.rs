/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Receipt : Transaction receipt

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(
        rename = "receiptUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_uid: Option<String>,
    #[serde(
        rename = "feedItemUid",
        skip_serializing_if = "Option::is_none"
    )]
    pub feed_item_uid: Option<String>,
    #[serde(rename = "metadataSource")]
    pub metadata_source: MetadataSource,
    #[serde(rename = "receiptIdentifier")]
    pub receipt_identifier: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: f32,
    #[serde(
        rename = "receiptMerchant",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_merchant: Option<Box<crate::models::ReceiptMerchant>>,
    /// ISO-4217 3 character currency code
    #[serde(
        rename = "currencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency_code: Option<CurrencyCode>,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::ReceiptItem>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<crate::models::ReceiptNote>>,
    #[serde(
        rename = "taxDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_details: Option<Vec<crate::models::ReceiptTaxDetail>>,
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Vec<crate::models::ReceiptPaymentMethod>,
    #[serde(
        rename = "providerName",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
}

impl Receipt {
    /// Transaction receipt
    pub fn new(
        metadata_source: MetadataSource,
        receipt_identifier: String,
        total_amount: f32,
        items: Vec<crate::models::ReceiptItem>,
        payment_methods: Vec<crate::models::ReceiptPaymentMethod>,
    ) -> Receipt {
        Receipt {
            receipt_uid: None,
            feed_item_uid: None,
            metadata_source,
            receipt_identifier,
            total_amount,
            receipt_merchant: None,
            currency_code: None,
            items,
            notes: None,
            tax_details: None,
            payment_methods,
            provider_name: None,
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
pub enum MetadataSource {
    #[serde(rename = "CUSTOMER")]
    CUSTOMER,
    #[serde(rename = "STARLING")]
    STARLING,
    #[serde(rename = "PARTNER")]
    PARTNER,
}

impl Default for MetadataSource {
    fn default() -> MetadataSource {
        Self::CUSTOMER
    }
}
/// ISO-4217 3 character currency code
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
pub enum CurrencyCode {
    #[serde(rename = "UNDEFINED")]
    UNDEFINED,
    #[serde(rename = "AED")]
    AED,
    #[serde(rename = "AFN")]
    AFN,
    #[serde(rename = "ALL")]
    ALL,
    #[serde(rename = "AMD")]
    AMD,
    #[serde(rename = "ANG")]
    ANG,
    #[serde(rename = "AOA")]
    AOA,
    #[serde(rename = "ARS")]
    ARS,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "AWG")]
    AWG,
    #[serde(rename = "AZN")]
    AZN,
    #[serde(rename = "BAM")]
    BAM,
    #[serde(rename = "BBD")]
    BBD,
    #[serde(rename = "BDT")]
    BDT,
    #[serde(rename = "BGN")]
    BGN,
    #[serde(rename = "BHD")]
    BHD,
    #[serde(rename = "BIF")]
    BIF,
    #[serde(rename = "BMD")]
    BMD,
    #[serde(rename = "BND")]
    BND,
    #[serde(rename = "BOB")]
    BOB,
    #[serde(rename = "BOV")]
    BOV,
    #[serde(rename = "BRL")]
    BRL,
    #[serde(rename = "BSD")]
    BSD,
    #[serde(rename = "BTN")]
    BTN,
    #[serde(rename = "BWP")]
    BWP,
    #[serde(rename = "BYN")]
    BYN,
    #[serde(rename = "BYR")]
    BYR,
    #[serde(rename = "BZD")]
    BZD,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "CDF")]
    CDF,
    #[serde(rename = "CHE")]
    CHE,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "CHW")]
    CHW,
    #[serde(rename = "CLF")]
    CLF,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "CNY")]
    CNY,
    #[serde(rename = "COP")]
    COP,
    #[serde(rename = "COU")]
    COU,
    #[serde(rename = "CRC")]
    CRC,
    #[serde(rename = "CUC")]
    CUC,
    #[serde(rename = "CUP")]
    CUP,
    #[serde(rename = "CVE")]
    CVE,
    #[serde(rename = "CZK")]
    CZK,
    #[serde(rename = "DJF")]
    DJF,
    #[serde(rename = "DKK")]
    DKK,
    #[serde(rename = "DOP")]
    DOP,
    #[serde(rename = "DZD")]
    DZD,
    #[serde(rename = "EGP")]
    EGP,
    #[serde(rename = "ERN")]
    ERN,
    #[serde(rename = "ETB")]
    ETB,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "FJD")]
    FJD,
    #[serde(rename = "FKP")]
    FKP,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "GEL")]
    GEL,
    #[serde(rename = "GHS")]
    GHS,
    #[serde(rename = "GIP")]
    GIP,
    #[serde(rename = "GMD")]
    GMD,
    #[serde(rename = "GNF")]
    GNF,
    #[serde(rename = "GTQ")]
    GTQ,
    #[serde(rename = "GYD")]
    GYD,
    #[serde(rename = "HKD")]
    HKD,
    #[serde(rename = "HNL")]
    HNL,
    #[serde(rename = "HRK")]
    HRK,
    #[serde(rename = "HTG")]
    HTG,
    #[serde(rename = "HUF")]
    HUF,
    #[serde(rename = "IDR")]
    IDR,
    #[serde(rename = "ILS")]
    ILS,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "IQD")]
    IQD,
    #[serde(rename = "IRR")]
    IRR,
    #[serde(rename = "ISK")]
    ISK,
    #[serde(rename = "JMD")]
    JMD,
    #[serde(rename = "JOD")]
    JOD,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "KES")]
    KES,
    #[serde(rename = "KGS")]
    KGS,
    #[serde(rename = "KHR")]
    KHR,
    #[serde(rename = "KMF")]
    KMF,
    #[serde(rename = "KPW")]
    KPW,
    #[serde(rename = "KRW")]
    KRW,
    #[serde(rename = "KWD")]
    KWD,
    #[serde(rename = "KYD")]
    KYD,
    #[serde(rename = "KZT")]
    KZT,
    #[serde(rename = "LAK")]
    LAK,
    #[serde(rename = "LBP")]
    LBP,
    #[serde(rename = "LKR")]
    LKR,
    #[serde(rename = "LRD")]
    LRD,
    #[serde(rename = "LSL")]
    LSL,
    #[serde(rename = "LTL")]
    LTL,
    #[serde(rename = "LYD")]
    LYD,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "MDL")]
    MDL,
    #[serde(rename = "MGA")]
    MGA,
    #[serde(rename = "MKD")]
    MKD,
    #[serde(rename = "MMK")]
    MMK,
    #[serde(rename = "MNT")]
    MNT,
    #[serde(rename = "MOP")]
    MOP,
    #[serde(rename = "MRO")]
    MRO,
    #[serde(rename = "MRU")]
    MRU,
    #[serde(rename = "MUR")]
    MUR,
    #[serde(rename = "MVR")]
    MVR,
    #[serde(rename = "MWK")]
    MWK,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "MXV")]
    MXV,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "MZN")]
    MZN,
    #[serde(rename = "NAD")]
    NAD,
    #[serde(rename = "NGN")]
    NGN,
    #[serde(rename = "NIO")]
    NIO,
    #[serde(rename = "NOK")]
    NOK,
    #[serde(rename = "NPR")]
    NPR,
    #[serde(rename = "NZD")]
    NZD,
    #[serde(rename = "OMR")]
    OMR,
    #[serde(rename = "PAB")]
    PAB,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "PGK")]
    PGK,
    #[serde(rename = "PHP")]
    PHP,
    #[serde(rename = "PKR")]
    PKR,
    #[serde(rename = "PLN")]
    PLN,
    #[serde(rename = "PYG")]
    PYG,
    #[serde(rename = "QAR")]
    QAR,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "RSD")]
    RSD,
    #[serde(rename = "RUB")]
    RUB,
    #[serde(rename = "RUR")]
    RUR,
    #[serde(rename = "RWF")]
    RWF,
    #[serde(rename = "SAR")]
    SAR,
    #[serde(rename = "SBD")]
    SBD,
    #[serde(rename = "SCR")]
    SCR,
    #[serde(rename = "SDG")]
    SDG,
    #[serde(rename = "SEK")]
    SEK,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "SHP")]
    SHP,
    #[serde(rename = "SLL")]
    SLL,
    #[serde(rename = "SOS")]
    SOS,
    #[serde(rename = "SRD")]
    SRD,
    #[serde(rename = "SSP")]
    SSP,
    #[serde(rename = "STD")]
    STD,
    #[serde(rename = "STN")]
    STN,
    #[serde(rename = "SVC")]
    SVC,
    #[serde(rename = "SYP")]
    SYP,
    #[serde(rename = "SZL")]
    SZL,
    #[serde(rename = "THB")]
    THB,
    #[serde(rename = "TJS")]
    TJS,
    #[serde(rename = "TMT")]
    TMT,
    #[serde(rename = "TND")]
    TND,
    #[serde(rename = "TOP")]
    TOP,
    #[serde(rename = "TRY")]
    _TRY,
    #[serde(rename = "TTD")]
    TTD,
    #[serde(rename = "TWD")]
    TWD,
    #[serde(rename = "TZS")]
    TZS,
    #[serde(rename = "UAH")]
    UAH,
    #[serde(rename = "UGX")]
    UGX,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "USN")]
    USN,
    #[serde(rename = "USS")]
    USS,
    #[serde(rename = "UYI")]
    UYI,
    #[serde(rename = "UYU")]
    UYU,
    #[serde(rename = "UZS")]
    UZS,
    #[serde(rename = "VEF")]
    VEF,
    #[serde(rename = "VES")]
    VES,
    #[serde(rename = "VND")]
    VND,
    #[serde(rename = "VUV")]
    VUV,
    #[serde(rename = "WST")]
    WST,
    #[serde(rename = "XAF")]
    XAF,
    #[serde(rename = "XAG")]
    XAG,
    #[serde(rename = "XAU")]
    XAU,
    #[serde(rename = "XBA")]
    XBA,
    #[serde(rename = "XBB")]
    XBB,
    #[serde(rename = "XBC")]
    XBC,
    #[serde(rename = "XBD")]
    XBD,
    #[serde(rename = "XCD")]
    XCD,
    #[serde(rename = "XDR")]
    XDR,
    #[serde(rename = "XOF")]
    XOF,
    #[serde(rename = "XPD")]
    XPD,
    #[serde(rename = "XPF")]
    XPF,
    #[serde(rename = "XPT")]
    XPT,
    #[serde(rename = "XSU")]
    XSU,
    #[serde(rename = "XTS")]
    XTS,
    #[serde(rename = "XUA")]
    XUA,
    #[serde(rename = "XXX")]
    XXX,
    #[serde(rename = "YER")]
    YER,
    #[serde(rename = "ZAR")]
    ZAR,
    #[serde(rename = "ZMW")]
    ZMW,
    #[serde(rename = "ZWL")]
    ZWL,
}

impl Default for CurrencyCode {
    fn default() -> CurrencyCode {
        Self::UNDEFINED
    }
}