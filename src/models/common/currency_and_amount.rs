use std::string::ToString;

#[derive(Clone, Default, Debug)]
pub struct CurrencyAndAmount {
    currency: String,
    minor_units: i32,
}

impl CurrencyAndAmount {
    #[allow(clippy::clippy::must_use_candidate)]
    pub fn new(currency: &str, minor_units: i32) -> CurrencyAndAmount {
        CurrencyAndAmount {
            currency: currency.to_owned(),
            minor_units,
        }
    }

    #[allow(clippy::clippy::must_use_candidate)]
    pub fn get_currency(&self) -> String {
        self.currency.clone()
    }

    #[allow(clippy::clippy::must_use_candidate)]
    pub fn get_minor_units(&self) -> i32 {
        self.minor_units
    }

    #[allow(clippy::clippy::must_use_candidate)]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_conventional_units(&self) -> f64 {
        f64::from(self.minor_units) / f64::from(100)
    }
}

impl ToString for CurrencyAndAmount {
    fn to_string(&self) -> String {
        format!("{} {}", self.to_conventional_units(), self.currency)
    }
}

#[cfg(test)]
mod test {
    use super::CurrencyAndAmount;

    #[test]
    pub fn test_fmt() {
        let amount = CurrencyAndAmount::new("GBP", 101_921);

        assert_eq!(amount.currency, "GBP");
        assert_eq!(amount.minor_units, 101_921);

        assert_eq!(amount.to_conventional_units(), 1019.21);
        assert_eq!(amount.to_string(), "1019.21 GBP");
    }
}
