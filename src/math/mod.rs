/// A collision extension focusing on 2d rectangles
pub mod geometry;

/// Mathematical equations
pub mod expressions;

//#[cfg(feature = "std")]
mod uniform_range;
//#[cfg(feature = "std")]
pub use uniform_range::*;

// /// Structs for positioning data in 2d and 3d space
// pub mod positioning;

/// Sets the length of the vector to 1 changing the direction it's facing
pub fn normalize_vector<T: NumberWithMonotoneOps + Abs + Sqrt + Copy>(
    x: T,
    y: T,
    z: T,
) -> (T, T, T) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    (x / v, y / v, z / v)
}

/// A trait for defining a number
pub const trait NumberWithMonotoneOps:
    core::cmp::PartialOrd
    + core::marker::Sized
    + [const] core::ops::Add<Output = Self>
    + [const] core::ops::Sub<Output = Self>
    + [const] core::ops::Mul<Output = Self>
    + [const] core::ops::Div<Output = Self>
    + [const] core::ops::Rem<Output = Self>
{
}

// What's up with this formatting?
impl<
    T: core::cmp::PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>,
> NumberWithMonotoneOps for T
{
}

use mirl_extensions::*;

/// A trait for simple but useful operations that weirdly enough do not exist in std
pub const trait ConvenientOps:
    Bounded + Copy + core::cmp::PartialOrd
{
    /// Get the half of a value
    #[must_use]
    fn half(&self) -> Self;
    #[must_use]
    /// Get the double of the current value
    fn double(&self) -> Self;
    #[must_use]
    /// Checks if a value is more than half its maximum
    fn more_than_half(&self) -> bool;
    #[must_use]
    /// Checks if a value is less than half its maximum
    fn less_than_half(&self) -> bool;
    #[must_use]
    /// Checks if a value is half its maximum
    fn equals_half(&self) -> bool;
}

impl<
    T: [const] ConstNumbers128
        + [const] NumberWithMonotoneOps
        + Copy
        + [const] Bounded
        + [const] PartialOrd,
> const ConvenientOps for T
{
    fn half(&self) -> Self {
        *self / Self::CONST_2
    }
    fn more_than_half(&self) -> bool {
        *self > Self::half(&Self::MAX)
    }
    fn less_than_half(&self) -> bool {
        *self < Self::half(&Self::MAX)
    }
    fn equals_half(&self) -> bool {
        *self == Self::half(&Self::MAX)
    }
    fn double(&self) -> Self {
        *self * Self::CONST_2
    }
}

/// Progress must be between 0 and 1 for this to work as intended most of the times
pub const fn interpolate<
    T: Copy
        + ConstOne
        + [const] core::ops::Add<Output = T>
        + [const] core::ops::Sub<Output = T>
        + [const] core::ops::Mul<Output = T>,
>(
    start: T,
    end: T,
    progress: T,
) -> T {
    start * (T::ONE - progress) + end * progress
}
/// Get the position of area A if you wanted to center it in area B
pub fn get_center_position_of_object_for_object<
    T: core::ops::Div<Output = T> + core::ops::Sub<Output = T> + ConstNumbers128,
>(
    inner_width: T,
    inner_height: T,
    outer_width: T,
    outer_height: T,
) -> (T, T) {
    // This is one hell of a way of getting the number 2 for a type
    (
        outer_width.div(T::CONST_2) - inner_width.div(T::CONST_2),
        outer_height.div(T::CONST_2) - inner_height.div(T::CONST_2),
    )
}

// use core::{f32, f64};

// #[allow(missing_docs)]
// use crate::prelude::{Abs, Sqrt};
