use crate::prelude::*;

/// User Preferences relating to submission of transactions.
#[derive(
    Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Decimal,
}

impl Transaction {
    /// Instantiates a new Transaction user preference with the
    /// specified `default_deposit_guarantee` value.
    pub fn new(default_deposit_guarantee: Decimal) -> Self {
        Self {
            default_deposit_guarantee,
        }
    }
}

impl Default for Transaction {
    /// By default `1.0` is used.
    fn default() -> Self {
        Self {
            default_deposit_guarantee: Decimal::one(),
        }
    }
}

impl HasPlaceholder for Transaction {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(Decimal::try_from_str("0.975").unwrap())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(Decimal::try_from_str("0.765").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(Transaction::placeholder(), Transaction::placeholder());
        assert_eq!(
            Transaction::placeholder_other(),
            Transaction::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Transaction::placeholder(),
            Transaction::placeholder_other()
        );
    }

    #[test]
    fn get_decimal() {
        let value = Decimal::new("0.975".to_string()).unwrap();
        let sut = Transaction::new(value.clone());
        assert_eq!(sut.default_deposit_guarantee, value)
    }

    #[test]
    fn json_roundtrip() {
        let sut = Transaction::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "defaultDepositGuarantee": "0.975"
            }
            "#,
        )
    }

    #[test]
    fn default_is_1() {
        assert_eq!(
            Transaction::default().default_deposit_guarantee.to_string(),
            "1"
        );
    }
}
