//! Fixed-point resource representation for economic and mass conservation.
//!
//! Plain-text arithmetic: All quantities are tracked as 64-bit signed integers
//! representing micro-units (1 unit = 1,000,000 micro-units).
//! Floating-point values are strictly forbidden in core state accounting to eliminate drift.

use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use crate::error::DomainError;

/// Fixed-point integer representing an exact resource quantity in micro-units.
///
/// Scale: 1 unit = 1,000,000 micro-units (1e6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct ResourceAmount(pub i64);

impl ResourceAmount {
    /// Fixed-point scale factor (micro-units per whole unit).
    pub const SCALE: i64 = 1_000_000;

    /// Zero resource quantity.
    pub const ZERO: Self = Self(0);

    /// Single whole unit quantity (1,000,000 micro-units).
    pub const ONE: Self = Self(Self::SCALE);

    /// Constructs a `ResourceAmount` directly from raw micro-units.
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// Returns the raw 64-bit micro-unit value.
    #[must_use]
    pub const fn to_raw(self) -> i64 {
        self.0
    }

    /// Constructs a `ResourceAmount` from whole units, checking for overflow.
    ///
    /// # Errors
    /// Returns `DomainError::ResourceOverflow` if units * SCALE overflows i64.
    pub fn from_units(units: i64) -> Result<Self, DomainError> {
        units
            .checked_mul(Self::SCALE)
            .map(Self)
            .ok_or_else(|| DomainError::ResourceOverflow(format!("from_units({units}) overflowed")))
    }

    /// Converts the resource amount to f64 for display / presentation DTOs only.
    /// Never use this value in simulation state updates.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_units_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    /// Checked addition preventing integer overflow.
    ///
    /// # Errors
    /// Returns `DomainError::ResourceOverflow` if addition overflows.
    pub fn checked_add(self, other: Self) -> Result<Self, DomainError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or_else(|| DomainError::ResourceOverflow(format!("{} + {}", self.0, other.0)))
    }

    /// Checked subtraction preventing integer underflow.
    ///
    /// # Errors
    /// Returns `DomainError::ResourceOverflow` if subtraction overflows/underflows i64.
    pub fn checked_sub(self, other: Self) -> Result<Self, DomainError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or_else(|| DomainError::ResourceOverflow(format!("{} - {}", self.0, other.0)))
    }

    /// Saturating addition.
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Saturating subtraction.
    #[must_use]
    pub fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /// Multiplies the amount by a rational fraction (numerator / denominator).
    /// Uses 128-bit intermediate arithmetic to prevent intermediate overflow.
    ///
    /// # Errors
    /// Returns `DomainError::ResourceOverflow` on division by zero or i64 overflow.
    pub fn checked_mul_ratio(self, numerator: i64, denominator: i64) -> Result<Self, DomainError> {
        if denominator == 0 {
            return Err(DomainError::ResourceOverflow("Division by zero in ratio".to_string()));
        }
        let wide_amount = i128::from(self.0);
        let wide_num = i128::from(numerator);
        let wide_den = i128::from(denominator);
        let product = wide_amount
            .checked_mul(wide_num)
            .ok_or_else(|| DomainError::ResourceOverflow("Ratio multiplication overflow".to_string()))?;
        let quotient = product / wide_den;

        i64::try_from(quotient)
            .map(Self)
            .map_err(|_| DomainError::ResourceOverflow("Ratio result out of i64 range".to_string()))
    }

    /// Validates that the amount is non-negative.
    ///
    /// # Errors
    /// Returns `DomainError::NegativeResourceAmount` if amount is strictly less than 0.
    pub fn ensure_non_negative(self) -> Result<Self, DomainError> {
        if self.0 < 0 {
            Err(DomainError::NegativeResourceAmount(self.0))
        } else {
            Ok(self)
        }
    }

    /// Returns true if the amount is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns true if the amount is strictly positive.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }
}

impl Add for ResourceAmount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ResourceAmount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for ResourceAmount {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ResourceAmount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<i64> for ResourceAmount {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i64> for ResourceAmount {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0 / Self::SCALE;
        let frac = (self.0 % Self::SCALE).abs();
        write!(f, "{whole}.{frac:06}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_point_conversions() {
        let amt = ResourceAmount::from_units(5).expect("valid conversion");
        assert_eq!(amt.to_raw(), 5_000_000);
        assert!((amt.to_units_f64() - 5.0).abs() < 1e-9);

        let zero = ResourceAmount::ZERO;
        assert_eq!(zero.to_raw(), 0);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_checked_arithmetic() {
        let a = ResourceAmount::from_raw(1_500_000);
        let b = ResourceAmount::from_raw(2_500_000);
        let sum = a.checked_add(b).expect("no overflow");
        assert_eq!(sum.to_raw(), 4_000_000);

        let diff = sum.checked_sub(a).expect("no underflow");
        assert_eq!(diff.to_raw(), 2_500_000);

        let max_val = ResourceAmount::from_raw(i64::MAX);
        assert!(max_val.checked_add(ResourceAmount::from_raw(1)).is_err());
    }

    #[test]
    fn test_checked_mul_ratio() {
        let base = ResourceAmount::from_units(10).expect("valid");
        // Multiply by 3/4 pro-rata
        let scaled = base.checked_mul_ratio(3, 4).expect("valid ratio");
        assert_eq!(scaled.to_raw(), 7_500_000);

        // Division by zero error
        assert!(base.checked_mul_ratio(1, 0).is_err());
    }

    #[test]
    fn test_non_negative_validation() {
        let positive = ResourceAmount::from_raw(100);
        assert!(positive.ensure_non_negative().is_ok());

        let zero = ResourceAmount::ZERO;
        assert!(zero.ensure_non_negative().is_ok());

        let negative = ResourceAmount::from_raw(-50);
        assert!(matches!(
            negative.ensure_non_negative(),
            Err(DomainError::NegativeResourceAmount(-50))
        ));
    }
}
