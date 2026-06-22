use crate::math::expressions::d2::GetY;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// x * m + n
pub struct LinearExpression<T> {
    /// The offset of x
    pub offset: T,
    /// The multiplier of x
    pub multiplier: T,
}

impl<T> LinearExpression<T> {
    #[must_use]
    /// Create a mew [`LinearExpression`] using a offset and multiplier
    pub const fn new(offset: T, multiplier: T) -> Self {
        Self {
            offset,
            multiplier,
        }
    }
}

impl<T: crate::math::NumberWithMonotoneOps + Copy> LinearExpression<T> {
    #[must_use]
    /// Get the linear expression from 2 known points
    pub fn new_from_2_points(point1: (T, T), point2: (T, T)) -> Self {
        let multiplier = (point2.1 - point1.1) / (point2.0 - point1.0);
        let offset = point1.1 - (multiplier * point1.0);
        Self {
            offset,
            multiplier,
        }
    }
}

impl<T: crate::math::NumberWithMonotoneOps + Copy> GetY<T>
    for LinearExpression<T>
{
    fn get_y(&self, x: T) -> T {
        x * self.multiplier + self.offset
    }
}
