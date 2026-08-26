#![allow(clippy::assign_op_pattern)]

use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    struct U512(8);
}

impl U256 {
    pub fn sqrt(self) -> U256 {
        if self.is_zero() {
            return U256::zero();
        }
        if self < U256::from(4) {
            return U256::from(1);
        }

        let mut x = self;
        let mut y = (x + U256::from(1)) >> 1;

        while y < x {
            x = y;
            y = (x + self / x) >> 1;
        }

        x
    }

    /// Compute `(self * b) / denominator` with full 512-bit precision on the intermediate product.
    /// Returns `U256::zero()` if `denominator` is zero.
    pub fn mul_div(self, b: U256, denominator: U256) -> U256 {
        if denominator.is_zero() {
            return U256::zero();
        }

        let a_512 = U512::from(self.low_u128()) | (U512::from(self.high_u128()) << 128);
        let b_512 = U512::from(b.low_u128()) | (U512::from(b.high_u128()) << 128);
        let d_512 =
            U512::from(denominator.low_u128()) | (U512::from(denominator.high_u128()) << 128);

        let result = (a_512 * b_512) / d_512;

        U256::from(result.low_u128()) | (U256::from((result >> 128).low_u128()) << 128)
    }

    /// Divide `self` by `b`, returning `U256::zero()` if `b` is zero.
    pub fn safe_div(self, b: U256) -> U256 {
        if b.is_zero() {
            return U256::zero();
        }
        self / b
    }

    /// Return the smaller of `self` and `other`.
    pub fn min(self, other: U256) -> U256 {
        if self < other {
            self
        } else {
            other
        }
    }

    /// Return the larger of `self` and `other`.
    pub fn max(self, other: U256) -> U256 {
        if self > other {
            self
        } else {
            other
        }
    }

    /// Calculate `self * numerator / base`.
    /// Uses 512-bit intermediate to avoid overflow.
    pub fn percent(self, numerator: u64, base: u64) -> U256 {
        self.mul_div(U256::from(numerator), U256::from(base))
    }

    fn high_u128(&self) -> u128 {
        let bytes = self.0;
        (bytes[2] as u128) | ((bytes[3] as u128) << 64)
    }
}

pub fn string_to_u256(input: String) -> U256 {
    U256::from_dec_str(input.as_str()).unwrap_or_default()
}

pub fn u256_to_string(value: U256) -> String {
    value.to_string()
}

pub fn is_valid_u256(input: String) -> bool {
    U256::from_dec_str(input.as_str()).is_ok()
}
