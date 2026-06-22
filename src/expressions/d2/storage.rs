// pub  const trait Expression<T>: SetFlippedExpression + GetY<T> {}

use crate::math::expressions::d2::{FlipExpression, GetY};

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// A struct holding x for the given expression
pub struct XHolder<T, Exp> {
    /// The current X
    pub x: T,
    /// The given expression on which [x](Self::x) is applied
    pub expression: Exp,
}
impl<T, E: GetY<T>> GetY<T> for XHolder<T, E> {
    fn get_y(&self, x: T) -> T {
        self.expression.get_y(x)
    }
}

impl<T: Clone, E: FlipExpression> FlipExpression for XHolder<T, E> {
    fn flip_horizontally(&self) -> Self {
        Self {
            x: self.x.clone(),
            expression: self.expression.flip_horizontally(),
        }
    }
    fn flip_vertically(&self) -> Self {
        Self {
            x: self.x.clone(),
            expression: self.expression.flip_vertically(),
        }
    }
}
impl<T, E> XHolder<T, E> {
    /// Creates a new [`XHolder`] from an x value and an expression.
    pub const fn new(x: T, expression: E) -> Self {
        Self {
            x,
            expression,
        }
    }

    /// Returns a new [`XHolder`] with the same expression but a different [x](Self::x) value.
    pub fn with_x<U>(self, x: U) -> XHolder<U, E> {
        XHolder {
            x,
            expression: self.expression,
        }
    }

    /// Transforms the inner expression while keeping the [x](Self::x) value unchanged.
    pub fn map_expression<F, E2>(self, f: F) -> XHolder<T, E2>
    where
        F: FnOnce(E) -> E2,
    {
        XHolder {
            x: self.x,
            expression: f(self.expression),
        }
    }
}
impl<T, E> AsRef<E> for XHolder<T, E> {
    fn as_ref(&self) -> &E {
        &self.expression
    }
}

#[cfg(feature = "std")]
impl<T, E> std::borrow::Borrow<E> for XHolder<T, E> {
    fn borrow(&self) -> &E {
        &self.expression
    }
}
