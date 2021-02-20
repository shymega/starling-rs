use std::string::ToString;

#[derive(Clone, Debug)]
pub struct CurrencyAndAmount {
    pub currency: String,
    pub minor_units: i64,
}

impl CurrencyAndAmount {
    #[allow(clippy::clippy::must_use_candidate)]
    pub fn new(currency: &str, minor_units: i64) -> CurrencyAndAmount {
        CurrencyAndAmount {
            currency: currency.to_owned(),
            minor_units,
        }
    }

    #[allow(clippy::clippy::must_use_candidate)]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_conventional_units(&self) -> f64 {
        (self.minor_units / 100) as f64
    }
}

impl ToString for CurrencyAndAmount {
    fn to_string(&self) -> String {
        format!("{} {}", self.to_conventional_units(), self.currency)
    }
}

impl Default for CurrencyAndAmount {
    #[allow(clippy::clippy::must_use_candidate)]
    fn default() -> CurrencyAndAmount {
        CurrencyAndAmount {
            currency: "XXX".to_owned(),
            minor_units: 0,
        }
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
