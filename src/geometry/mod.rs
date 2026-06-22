use mirl_extensions::ShapeDirectionType;

/// When the bottom of the collision is mathematically higher
pub const BOTTOM_HIGHER: bool = true;
/// When the top of the collision is mathematically higher
pub const BOTTOM_LOWER: bool = false;

// impl<T> Shape<T> for EmptyShape {}
/// When the given position is at an edge or corner within the range, return the type of the closest [`ShapeDirectionType`](crate::directions::ShapeDirectionType)
pub const trait GetShapeDirectionType<T, R: ShapeDirectionType> {
    /// Get the edge type for the closest edge/corner
    fn get_edge_or_corner_type(&self, point: (T, T), margin: T) -> R;
    /// Get the closest edge type
    fn get_edge_type(&self, point: (T, T), margin: T) -> R;
    /// Get the closest corner type
    fn get_corner_type(&self, point: (T, T), margin: T) -> R;
}
/// Asks if two shapes would collide with one another
pub const trait DoShapesCollide<Other> {
    /// Does this shape collide with another?
    fn collides_with_shape(&self, other: &Other) -> bool;
}
impl<S, T: DoShapesCollide<S>> DoShapesCollide<T> for S {
    default fn collides_with_shape(&self, other: &T) -> bool {
        other.collides_with_shape(self)
    }
}
/// 2D geometry
pub mod d2;
