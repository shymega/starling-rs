#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum AccountHolderType {
    INDIVIDUAL,
    BUSINESS,
    SOLE_TRADER,
    JOINT,
    BANKING_AS_A_SERVICE,
    UNDEFINED,
}

impl Default for AccountHolderType {
    fn default() -> AccountHolderType {
        AccountHolderType::UNDEFINED
    }
}
