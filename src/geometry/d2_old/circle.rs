use mirl_extensions::*;

use crate::math::{
    geometry::{positioning::*, Pos2D, Shape},
    ConvenientOps, NumberWithMonotoneOps,
};

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A simple Rectangle defining computational limits
#[allow(missing_docs)]
pub struct Circle<T, const CS: bool> {
    pub radius: T,
    pub half_radius: T,
}
impl<const CS: bool, T: NumberWithMonotoneOps + Copy + ConvenientOps>
    Circle<T, CS>
{
    #[allow(missing_docs)]
    pub fn new(radius: T) -> Self {
        Self {
            radius,
            half_radius: radius.half(),
        }
    }
}
impl<T, const CS: bool> Shape<T> for Circle<T, CS> {}

impl<const CS: bool, T: NumberWithMonotoneOps + Copy> Pos2D<T, Circle<T, CS>> {
    /// Checks if a point is inside the radius of the circle
    pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
        let dx = point.0 - (self.get_x() + self.get_shape().half_radius);
        let dy = point.1 - (self.get_x() + self.get_shape().half_radius);
        dx * dx + dy * dy <= self.get_shape().radius * self.get_shape().radius
    }
}

impl<const CS: bool, T: NumberWithMonotoneOps + Copy + ConstZero + Sqrt>
    Pos2D<T, Circle<T, CS>>
{
    /// Get the closest point on the edge to the defined point
    pub fn get_closest_point_on_edge(&self, point: (T, T)) -> (T, T) {
        let cx = self.get_x() + self.get_shape().half_radius;
        let cy = self.get_y() + self.get_shape().half_radius;
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        let dist_sq = dx * dx + dy * dy;
        if core::intrinsics::unlikely(dist_sq == T::ZERO) {
            return (cx + self.get_shape().radius, cy); // arbitrary edge point when point == center
        }
        let dist = dist_sq.sqrt();
        let scale = self.get_shape().radius / dist;
        (cx + dx * scale, cy + dy * scale)
    }
}
