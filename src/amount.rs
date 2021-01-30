use rust_decimal::Decimal;

const DECIMAL_PLACES: u32 = 4;

#[derive(Copy, Clone, PartialEq)]
pub struct Amount {
    value: Decimal,
}

impl Amount {
    /// Creates a new Amount with 4 decimal places.
    pub fn new(value: i64) -> Self {
        Self {
            value: Decimal::new(value, DECIMAL_PLACES),
        }
    }

    fn base_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }

    /// An amount set to 0.
    pub fn zero() -> Self {
        Self::new(0)
    }

    /// Checks whether the amount is less than 0
    pub fn less_than_zero(&self) -> bool {
        self.value < Self::zero().value
    }
}

impl std::ops::Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Debug for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base_fmt(f)
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base_fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_display_returns_expected() {
        let amount = Amount::new(314);
        let expected = "0.0314";
        let actual = format!("{}", amount);
        assert_eq!(expected, actual);

        let amount = Amount::new(-110023945800);
        let expected = "-11002394.5800";
        let actual = format!("{}", amount);
        assert_eq!(expected, actual);
    }

    #[test]
    fn amount_add_returns_expected() {
        let a = Amount::new(314);
        let b = Amount::new(100);

        assert_eq!(Amount::new(414), a + b);

        let a = Amount::new(314);
        let b = Amount::new(-1100);

        assert_eq!(Amount::new(-786), a + b);
    }

    #[test]
    fn amount_subtract_returns_expected() {
        let a = Amount::new(314);
        let b = Amount::new(100);

        assert_eq!(Amount::new(214), a - b);

        let a = Amount::new(314);
        let b = Amount::new(-1100);

        assert_eq!(Amount::new(1414), a - b);
    }

    #[test]
    fn amount_negative_one_less_than_zero_returns_true() {
        let amount = Amount {
            value: Decimal::new(-1, 4),
        };

        assert_eq!(true, amount.less_than_zero());
    }

    #[test]
    fn amount_one_less_than_zero_returns_false() {
        let amount = Amount {
            value: Decimal::new(1, 4),
        };

        assert_eq!(false, amount.less_than_zero());
    }

    #[test]
    fn amount_zero_returns_zero() {
        let expected = Amount {
            value: Decimal::new(0, 4),
        };
        let actual = Amount::zero();

        assert_eq!(expected, actual);
    }

    #[test]
    fn amount_default_returns_zero() {
        let expected = Amount {
            value: Decimal::new(0, 4),
        };
        let actual = Amount::default();

        assert_eq!(expected, actual);
    }
}
