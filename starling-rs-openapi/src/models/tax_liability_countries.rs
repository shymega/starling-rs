/*
 * Starling Bank API
 *
 * OpenAPI specification for the Starling Bank Public API.  For more information visit: https://developer.starlingbank.com/docs
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// TaxLiabilityCountries : List of countries that can be submitted in a tax declaration

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxLiabilityCountries {
    #[serde(
        rename = "countries",
        skip_serializing_if = "Option::is_none"
    )]
    pub countries: Option<Vec<Countries>>,
}

impl TaxLiabilityCountries {
    /// List of countries that can be submitted in a tax declaration
    pub fn new() -> TaxLiabilityCountries {
        TaxLiabilityCountries { countries: None }
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
pub enum Countries {
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

impl Default for Countries {
    fn default() -> Countries {
        Self::UNDEFINED
    }
}
