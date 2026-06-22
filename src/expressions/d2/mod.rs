/// Linear expression math
pub mod linear;

/// A generic storage holder for automatically dealing with x
pub mod storage;

/// Get the y coord given the x
pub const trait GetY<T> {
    #[must_use]
    /// Get the y coord given the x
    fn get_y(&self, x: T) -> T;
}
#[cfg(feature = "std")]
/// Get all collisions of the given expressions
pub const trait GetCollisions<E, T> {
    #[must_use]
    /// Get all collisions of the given expressions
    ///
    /// Q: Why `Option<Vec<T>>` instead of `Vec<T>?`
    ///
    /// A: Some expressions combinations are invalid for collision detection like equal expressions which will return `None` instead of an empty `Vec`
    fn get_collisions(&self, other: &E) -> Option<Vec<T>>;
}
#[cfg(feature = "std")]
impl<E, T, S: GetCollisions<E, T>> GetCollisions<S, T> for E {
    fn get_collisions(&self, other: &S) -> Option<Vec<T>> {
        other.get_collisions(self)
    }
}
/// Flip the given expression vertically or horizontally
pub const trait FlipExpression {
    #[must_use]
    /// Flip the given expression vertically
    fn flip_vertically(&self) -> Self;
    #[must_use]
    /// Flip the given expression horizontally
    fn flip_horizontally(&self) -> Self;
}
/// Set the current expression to a flipped version
pub const trait SetFlippedExpression: FlipExpression {
    /// Flip the given expression vertically
    fn set_flipped_vertically(&mut self);
    /// Flip the given expression horizontally
    fn set_flipped_horizontally(&mut self);
}
impl<T: FlipExpression> SetFlippedExpression for T {
    fn set_flipped_horizontally(&mut self) {
        *self = self.flip_horizontally();
    }
    fn set_flipped_vertically(&mut self) {
        *self = self.flip_vertically();
    }
}
