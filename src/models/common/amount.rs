#[derive(Default, Clone, Debug)]
pub struct Amount {
    pub currency: String,
    pub minor_units: f64,
}

impl Amount {
    pub fn new(currency: String, minor_units: i64) -> Amount {
        Amount {
            currency: currency,
            minor_units: minor_units as f64,
        }
    }

    pub fn to_conventional_units(&self) -> f64 {
        self.minor_units / 100 as f64
    }

    pub fn to_fmt_string(&self) -> String {
        format!("{} {}", self.to_conventional_units(), self.currency)
    }
}

#[cfg(test)]
mod test {
    use super::Amount;

    #[test]
    pub fn test_fmt() {
        let amount = Amount::new("GBP".to_owned(), 101921);

        assert_eq!(amount.currency, "GBP");
        assert_eq!(amount.minor_units, 101921 as f64);

        assert_eq!(amount.to_conventional_units(), 1019.21);
        assert_eq!(amount.to_fmt_string(), "1019.21 GBP");
    }
}