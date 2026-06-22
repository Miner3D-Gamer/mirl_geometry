// Not generally a fan of ai written code in most cases but when the 1 in a million prompt hits they produce some pretty workin' code
use core::ops::{
    Add,
    //AddAssign,
    Div,
    //DivAssign,
    Mul,
    //MulAssign,
    Sub,
    //SubAssign,
};

use mirl_extensions::*;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A type that represents a uniform range from 0.0 to 1.0 using any unsigned integer type
/// The internal value is stored as an unsigned integer where 0 represents 0.0 and MAX represents 1.0
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniformRange<T: Copy + core::fmt::Debug> {
    value: T,
    //_phantom: core::marker::PhantomData<T>,
}

#[allow(unused_imports)]
use mirl_extensions::*;

impl<T> UniformRange<T>
where
    T: TryIntoPatch<f64>
        + Copy
        + ConstZero
        + ConstOne
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + core::fmt::Debug
        + Div<Output = T>,
    f64: TryIntoPatch<T>,
{
    #[must_use]
    /// Create a new `UnitRange` with value 0.0
    pub const fn zero() -> Self {
        Self {
            value: T::ZERO,
            //_phantom: core::marker::PhantomData,
        }
    }

    #[must_use]
    /// Create a new `UnitRange` with value 1.0
    pub fn one() -> Self {
        Self {
            value: Self::max_value(),
            //_phantom: core::marker::PhantomData,
        }
    }

    #[must_use]
    /// Create a new `UnitRange` from a raw integer value
    pub const fn from_raw(value: T) -> Self {
        Self {
            value,
            //_phantom: core::marker::PhantomData,
        }
    }

    #[must_use]
    /// Create a new `UnitRange` from a float value (0.0 to 1.0)
    ///
    /// # Panics
    /// If T doesn't support being cast from f64
    pub fn from_f64(f: f64) -> Option<Self> {
        let clamped = f.clamp(0.0, 1.0);
        let max_val: f64 = (Self::max_value()).try_into_value()?;
        let scaled = (clamped * max_val).round();
        let value: T = (scaled).try_into_value()?;
        Some(Self {
            value,
            //_phantom: core::marker::PhantomData,
        })
    }
    #[must_use]
    #[allow(clippy::cast_lossless)]
    /// Create a new `UnitRange` from a float value (0.0 to 1.0)
    pub fn from_f32(f: f32) -> Option<Self> {
        Self::from_f64(f as f64)
    }
    #[must_use]
    #[allow(clippy::cast_lossless)]
    /// Create a new `UnitRange` from a float value (0.0 to 1.0)
    pub fn from_f16(f: f16) -> Option<Self> {
        Self::from_f64(f as f64)
    }
    #[must_use]
    /// Get the raw integer value
    pub const fn raw(&self) -> T {
        self.value
    }

    #[must_use]
    /// Convert to f64 (0.0 to 1.0)
    pub fn to_f64(&self) -> Option<f64> {
        let val: f64 = (self.value).try_into_value()?;
        let max_val: f64 = (Self::max_value()).try_into_value()?;
        Some(val / max_val)
    }
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Convert to f32 (0.0 to 1.0)
    pub fn to_f32(&self) -> Option<f32> {
        self.to_f64().and_then(f32::try_from_value)
    }
    #[must_use]
    /// Get the maximum value for the underlying type
    #[allow(arithmetic_overflow)]
    fn max_value() -> T {
        // We need to compute the maximum value for the unsigned type
        // For unsigned types, this is typically 2^n - 1
        let zero = T::ZERO;
        let one = T::ONE;

        // Find the max by bit shifting or using a more direct approach
        // This is a bit tricky generically, so we'll use NumCast with known max values
        // if std::mem::size_of::<T>() == 1 {
        //     NumCast::from(u8::MAX).unwrap()
        // } else if std::mem::size_of::<T>() == 2 {
        //     NumCast::from(u16::MAX).unwrap()
        // } else if std::mem::size_of::<T>() == 4 {
        //     NumCast::from(u32::MAX).unwrap()
        // } else if std::mem::size_of::<T>() == 8 {
        //     NumCast::from(u64::MAX).unwrap()
        // } else if std::mem::size_of::<T>() == 16 {
        //     NumCast::from(u128::MAX).unwrap()
        // } else
        {
            // Fallback: try to compute max value
            let mut max_val = zero;
            let mut temp = one;

            // Simple approach: keep doubling until we wrap around
            loop {
                let next = temp + temp;
                if next < temp {
                    // Overflow detected
                    break;
                }
                temp = next;
            }

            // Now find the exact max by adding smaller increments
            while temp + one >= temp {
                max_val = temp;
                temp = temp + one;
            }

            max_val
        }
    }
    #[must_use]
    /// Saturating addition
    pub fn saturating_add(self, other: Self) -> Self {
        let max_val = Self::max_value();
        let result = if self.value > max_val - other.value {
            max_val
        } else {
            self.value + other.value
        };
        Self::from_raw(result)
    }
    #[must_use]
    /// Saturating subtraction
    pub fn saturating_sub(self, other: Self) -> Self {
        let result = if self.value < other.value {
            T::ZERO
        } else {
            self.value - other.value
        };
        Self::from_raw(result)
    }
}

// Arithmetic operations
impl<T> Add for UniformRange<T>
where
    T: Copy
        + ConstZero
        + ConstOne
        + core::fmt::Debug
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + TryIntoPatch<f64>,
    f64: TryIntoPatch<T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl<T> Sub for UniformRange<T>
where
    T: TryIntoPatch<f64>
        + Copy
        + ConstZero
        + ConstOne
        + PartialOrd
        + core::fmt::Debug
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
    f64: TryIntoPatch<T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

// impl<T> Mul for UniformRange<T>
// where
//     T: Unsigned
//         + TryFromPatch<f64>
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
//     f64: TryFromPatch<T>,
//     Self: TryFromPatch<f64>,
// {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Option<Self::Output> {
//         // Convert to f64, multiply, then convert back
//         let result = self.to_f64() * rhs.to_f64();
//         Self::try_from_value(result)
//     }
// }

// impl<T> Div for UniformRange<T>
// where
//     T: Unsigned
//         + NumCast
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
// {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {
//         if rhs.value == T::zero() {
//             return Self::ONE; // Division by zero gives max value (1.0)
//         }

//         // Convert to f64, divide, then convert back
//         let result = self.to_f64() / rhs.to_f64();
//         Self::from_f64(result)
//     }
// }

// // Assignment operations
// impl<T> AddAssign for UniformRange<T>
// where
//     T: Unsigned
//         + NumCast
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
// {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }

// impl<T> SubAssign for UniformRange<T>
// where
//     T: Unsigned
//         + NumCast
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
// {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }

// impl<T> MulAssign for UniformRange<T>
// where
//     T: Unsigned
//         + NumCast
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
// {
//     fn mul_assign(&mut self, rhs: Self) {
//         *self = *self * rhs;
//     }
// }

// impl<T> DivAssign for UniformRange<T>
// where
//     T: Unsigned
//         + NumCast
//         + Copy
//         + Zero
//         + One
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>,
// {
//     fn div_assign(&mut self, rhs: Self) {
//         *self = *self / rhs;
//     }
// }

impl<T> core::fmt::Display for UniformRange<T>
where
    T: TryIntoPatch<f64>
        + Copy
        + ConstZero
        + ConstOne
        + PartialOrd
        + core::fmt::Debug
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
    f64: TryIntoPatch<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.6?}", self.to_f64())
    }
}

impl<T> core::fmt::Debug for UniformRange<T>
where
    T: TryIntoPatch<f64>
        + Copy
        + ConstZero
        + ConstOne
        + PartialOrd
        + core::fmt::Debug
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
    f64: TryIntoPatch<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "UnitRange(raw: {:?}, value: {:.6?})",
            self.value,
            self.to_f64()
        )
    }
}

/// Unit range with u8 storage - 256 values, lowest precision, smallest memory footprint.
pub type UnitRangeU8 = UniformRange<u8>;

/// Unit range with u16 storage - 65K values, moderate precision, good balance for most uses.
pub type UnitRangeU16 = UniformRange<u16>;

/// Unit range with u32 storage - 4.3B values, high precision for fine-grained control.
pub type UnitRangeU32 = UniformRange<u32>;

/// Unit range with u64 storage - 18.4Q values, maximum precision for scientific computing.
pub type UnitRangeU64 = UniformRange<u64>;

/// Unit range with u128 storage - 340 undecillion values, go ahead.
pub type UnitRangeU128 = UniformRange<u128>;

/// Unit range with usize storage - platform-dependent precision (32-bit or 64-bit).
pub type UnitRangeUsize = UniformRange<usize>;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic_functionality() {
//         let zero = UnitRangeU8::ZERO;
//         let one = UnitRangeU8::ONE;
//         let half = UnitRangeU8::from_f64(0.5);

//         assert_eq!(zero.to_f64(), 0.0);
//         assert_eq!(one.to_f64(), 1.0);
//         assert!((half.to_f64() - 0.5).abs() < 0.01);
//     }

//     #[test]
//     fn test_arithmetic() {
//         let a = UnitRangeU8::from_f64(0.3);
//         let b = UnitRangeU8::from_f64(0.4);

//         let sum = a + b;
//         let diff = a - b;
//         let product = a * b;
//         let quotient = a / b;

//         assert!((sum.to_f64() - 0.7).abs() < 0.01);
//         assert!(diff.to_f64() < 0.01); // Should be close to 0 due to saturating sub
//         assert!((product.to_f64() - 0.12).abs() < 0.01);
//         assert!((quotient.to_f64() - 0.75).abs() < 0.01);
//     }

//     #[test]
//     fn test_different_types() {
//         let u8_val = UnitRangeU8::from_f64(0.5);
//         let u16_val = UnitRangeU16::from_f64(0.5);
//         let u32_val = UnitRangeU32::from_f64(0.5);

//         // u16 should have more precision than u8
//         assert!(
//             (u16_val.to_f64() - 0.5).abs() <= (u8_val.to_f64() - 0.5).abs()
//         );
//         assert!(
//             (u32_val.to_f64() - 0.5).abs() <= (u16_val.to_f64() - 0.5).abs()
//         );
//     }

//     #[test]
//     fn test_saturation() {
//         let max = UnitRangeU8::ONE;
//         let small = UnitRangeU8::from_f64(0.1);

//         let saturated_add = max + small;
//         let saturated_sub = small - max;

//         assert_eq!(saturated_add.to_f64(), 1.0);
//         assert_eq!(saturated_sub.to_f64(), 0.0);
//     }
// }
