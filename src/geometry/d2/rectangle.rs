use mirl_extensions::*;

use crate::math::geometry::GetShapeDirectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A simple Rectangle defining computational limits, with its own position.
///
/// The `BOTTOM_HIGHER` const generic controls the coordinate system:
/// - `true`  → bottom has a *higher* numeric value than top (e.g. screen-space Y-down)
/// - `false` → top has a *higher* numeric value than bottom (e.g. math/world-space Y-up)
#[allow(missing_docs)]
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub struct Rectangle<T, const BOTTOM_HIGHER: bool> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

// ---------------------------------------------------------------------------
// Default
// ---------------------------------------------------------------------------

impl<T: Default, const BOTTOM_HIGHER: bool> Default
    for Rectangle<T, BOTTOM_HIGHER>
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: T::default(),
            height: T::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Construction & basic mutation
// ---------------------------------------------------------------------------

impl<T: Copy, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER> {
    /// Create a new `Rectangle` from a position and size.
    #[must_use]
    pub const fn new(pos: (T, T), size: (T, T)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            width: size.0,
            height: size.1,
        }
    }

    /// Create a new `Rectangle` from only a size (position defaults to the supplied `origin`).
    #[must_use]
    pub const fn from_size(size: (T, T), origin: (T, T)) -> Self {
        Self::new(origin, size)
    }

    /// Set the position of the rectangle.
    pub const fn set_pos(&mut self, pos: (T, T)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    /// Set the size of the rectangle.
    pub const fn set_size(&mut self, size: (T, T)) {
        self.width = size.0;
        self.height = size.1;
    }
}

// ---------------------------------------------------------------------------
// Getters (position & size)
// ---------------------------------------------------------------------------

impl<T: Copy, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER> {
    /// Get the current position `(x, y)`.
    #[must_use]
    #[inline(always)]
    pub const fn get_pos(&self) -> (T, T) {
        (self.x, self.y)
    }

    /// Get the current x coordinate.
    #[must_use]
    #[inline(always)]
    pub const fn get_x(&self) -> T {
        self.x
    }

    /// Get the current y coordinate.
    #[must_use]
    #[inline(always)]
    pub const fn get_y(&self) -> T {
        self.y
    }

    /// Get the current size `(width, height)`.
    #[must_use]
    #[inline(always)]
    pub const fn get_size(&self) -> (T, T) {
        (self.width, self.height)
    }

    /// Get the current width.
    #[must_use]
    #[inline(always)]
    pub const fn get_width(&self) -> T {
        self.width
    }

    /// Get the current height.
    #[must_use]
    #[inline(always)]
    pub const fn get_height(&self) -> T {
        self.height
    }
}

// ---------------------------------------------------------------------------
// Derived edge accessors
// ---------------------------------------------------------------------------

impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: core::ops::Add<Output = T> + PartialOrd + Copy,
{
    /// Return a copy of `self` translated by `offset_pos`.
    #[must_use]
    #[inline(always)]
    pub fn offset_pos(&self, offset_pos: (T, T)) -> Self {
        Self {
            x: self.x + offset_pos.0,
            y: self.y + offset_pos.1,
            width: self.width,
            height: self.height,
        }
    }

    /// Left edge x-coordinate.
    #[must_use]
    #[inline(always)]
    pub const fn left(&self) -> T {
        self.x
    }

    /// Right edge x-coordinate.
    #[must_use]
    #[inline(always)]
    pub const fn right(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        self.x + self.width
    }

    /// Top edge y-coordinate (respects coordinate-system convention).
    #[must_use]
    #[inline(always)]
    pub const fn top(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.y + self.height
        } else {
            self.y
        }
    }

    /// Bottom edge y-coordinate (respects coordinate-system convention).
    #[must_use]
    #[inline(always)]
    pub const fn bottom(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.y
        } else {
            self.y + self.height
        }
    }
}

// ---------------------------------------------------------------------------
// Ratio
// ---------------------------------------------------------------------------

impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: core::ops::Div<Output = T> + PartialOrd + Copy,
{
    /// The aspect ratio: `height / width`.
    #[must_use]
    #[inline(always)]
    pub fn get_ratio(&self) -> T {
        self.height / self.width
    }
}

// ---------------------------------------------------------------------------
// Center (requires ConstNumbers128 from mirl_extensions)
// ---------------------------------------------------------------------------

impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: ConstNumbers128
        + core::ops::Add<Output = T>
        + core::ops::Div<Output = T>
        + Copy,
{
    /// Get the center position of the rectangle.
    #[must_use]
    #[inline(always)]
    pub fn center(&self) -> (T, T) {
        (self.x + self.width / T::CONST_2, self.y + self.height / T::CONST_2)
    }
}

// ---------------------------------------------------------------------------
// mirl_extensions trait impls
// ---------------------------------------------------------------------------

impl<T: Copy, const BOTTOM_HIGHER: bool> Get2DCoordinate<T>
    for Rectangle<T, BOTTOM_HIGHER>
{
    #[inline(always)]
    fn get_pos_2d_x(&self) -> T {
        self.x
    }

    #[inline(always)]
    fn get_pos_2d_y(&self) -> T {
        self.y
    }

    #[inline(always)]
    fn set_pos_2d_x(&mut self, val: T) {
        self.x = val;
    }

    #[inline(always)]
    fn set_pos_2d_y(&mut self, val: T) {
        self.y = val;
    }
}

// `Get2DCoordinateHelper` is blanket-implemented for all `Get2DCoordinate`
// types, so `Rectangle` gets `get_pos_2d` / `set_pos_2d` for free.

impl<T, const BOTTOM_HIGHER: bool> Get2DCoordinateCenter<T>
    for Rectangle<T, BOTTOM_HIGHER>
where
    T: ConstNumbers128
        + core::ops::Add<Output = T>
        + core::ops::Div<Output = T>
        + Copy,
{
    #[inline(always)]
    fn get_pos_2d_of_center(&self) -> (T, T) {
        self.center()
    }
}

// ---------------------------------------------------------------------------
// Per-concrete-type macro (collision / containment / edge detection)
// ---------------------------------------------------------------------------

#[macro_export]
/// Automatically implement a bunch of rectangle math for the given primitive type.
macro_rules! impl_const_rectangle_for_type {
    ($t:ty) => {
        impl<const CS: bool> Rectangle<$t, CS> {
            /// Checks if two rectangles intersect (inclusive edges).
            #[must_use]
            #[inline(always)]
            pub const fn do_areas_intersect(
                &self,
                other: &Rectangle<$t, CS>,
            ) -> bool {
                self.left() <= other.right()
                    && self.right() >= other.left()
                    && if CS {
                        self.bottom() <= other.top()
                            && self.top() >= other.bottom()
                    } else {
                        self.top() <= other.bottom()
                            && self.bottom() >= other.top()
                    }
            }

            /// Checks if two rectangles intersect (exclusive edges).
            #[must_use]
            #[inline(always)]
            pub const fn do_areas_intersect_strict(
                &self,
                other: &Rectangle<$t, CS>,
            ) -> bool {
                self.left() < other.right()
                    && self.right() > other.left()
                    && if CS {
                        self.bottom() < other.top()
                            && self.top() > other.bottom()
                    } else {
                        self.top() < other.bottom()
                            && self.bottom() > other.top()
                    }
            }

            /// Checks if this area fully includes another area.
            #[must_use]
            #[inline]
            pub const fn does_area_fully_include_other_area(
                &self,
                other: &Rectangle<$t, CS>,
            ) -> bool {
                if CS {
                    self.left() <= other.left()
                        && self.right() >= other.right()
                        && self.bottom() <= other.bottom()
                        && self.top() >= other.top()
                } else {
                    self.left() <= other.left()
                        && self.right() >= other.right()
                        && self.bottom() >= other.bottom()
                        && self.top() <= other.top()
                }
            }

            /// Checks if a point falls within the coordinate range defined by
            /// the rectangle bounds (inclusive).
            #[must_use]
            #[inline(always)]
            pub const fn does_area_contain_point(
                &self,
                point: ($t, $t),
            ) -> bool {
                let (x, y) = point;
                x >= self.left()
                    && x <= self.right()
                    && if !CS {
                        y >= self.top() && y <= self.bottom()
                    } else {
                        y <= self.top() && y >= self.bottom()
                    }
            }

            /// Check if a point is within `margin` distance of any edge.
            #[must_use]
            #[inline(always)]
            pub const fn is_point_at_edge(
                &self,
                point: ($t, $t),
                margin: $t,
            ) -> bool {
                let (x, y) = point;
                let near_left =
                    x >= self.left() - margin && x <= self.left() + margin;
                let near_right =
                    x >= self.right() - margin && x <= self.right() + margin;
                let near_top =
                    y >= self.top() - margin && y <= self.top() + margin;
                let near_bottom =
                    y >= self.bottom() - margin && y <= self.bottom() + margin;
                near_left || near_right || near_top || near_bottom
            }

            /// Returns which edge/corner a point is near.
            ///
            /// - `EDGE`   – include edge directions
            /// - `CORNER` – include corner directions
            ///
            /// Returns `R::none_directional()` if not within `margin` of any edge.
            #[must_use]
            #[allow(clippy::too_many_lines)]
            pub const fn get_edge_position<
                const EDGE: bool,
                const CORNER: bool,
                R: [const] $crate::math::geometry::ShapeDirectionType,
            >(
                &self,
                point: ($t, $t),
                margin: $t,
            ) -> R {
                let (x, y) = point;
                let (near_left, near_right, near_top, near_bottom) = if CS {
                    let near_left = x >= self.left() - margin
                        && x <= self.left() + margin
                        && y >= self.bottom() - margin
                        && y <= self.top() + margin;
                    let near_right = x >= self.right() - margin
                        && x <= self.right() + margin
                        && y >= self.bottom() - margin
                        && y <= self.top() + margin;
                    let near_top = y >= self.bottom() - margin
                        && y <= self.bottom() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    let near_bottom = y >= self.top() - margin
                        && y <= self.top() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    (near_left, near_right, near_top, near_bottom)
                } else {
                    let near_left = x >= self.left() - margin
                        && x <= self.left() + margin
                        && y >= self.top() - margin
                        && y <= self.bottom() + margin;
                    let near_right = x >= self.right() - margin
                        && x <= self.right() + margin
                        && y >= self.top() - margin
                        && y <= self.bottom() + margin;
                    let near_top = y >= self.top() - margin
                        && y <= self.top() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    let near_bottom = y >= self.bottom() - margin
                        && y <= self.bottom() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    (near_left, near_right, near_top, near_bottom)
                };

                // Extended corner regions – diagonal extensions beyond the rectangle
                let (
                    in_top_left_direction_diagonal,
                    in_top_right_direction_diagonal,
                    in_bottom_right_direction_diagonal,
                    in_bottom_left_direction_diagonal,
                ) = if CS {
                    let in_top_left_direction_diagonal = (x < self.left()
                        && y > self.bottom())
                        && (self.left() - x + y - self.bottom()) <= margin;
                    let in_top_right_direction_diagonal = (x > self.right()
                        && y > self.bottom())
                        && (x - self.right() + y - self.bottom()) <= margin;
                    let in_bottom_right_direction_diagonal = (x > self.right()
                        && y < self.top())
                        && (x - self.right() + self.top() - y) <= margin;
                    let in_bottom_left_direction_diagonal = (x < self.left()
                        && y < self.top())
                        && (self.left() - x + self.top() - y) <= margin;
                    (
                        in_top_left_direction_diagonal,
                        in_top_right_direction_diagonal,
                        in_bottom_right_direction_diagonal,
                        in_bottom_left_direction_diagonal,
                    )
                } else {
                    let in_top_left_direction_diagonal = (x < self.left()
                        && y < self.top())
                        && (self.left() - x + self.top() - y) <= margin;
                    let in_top_right_direction_diagonal = (x > self.right()
                        && y < self.top())
                        && (x - self.right() + self.top() - y) <= margin;
                    let in_bottom_right_direction_diagonal = (x > self.right()
                        && y > self.bottom())
                        && (x - self.right() + y - self.bottom()) <= margin;
                    let in_bottom_left_direction_diagonal = (x < self.left()
                        && y > self.bottom())
                        && (self.left() - x + y - self.bottom()) <= margin;
                    (
                        in_top_left_direction_diagonal,
                        in_top_right_direction_diagonal,
                        in_bottom_right_direction_diagonal,
                        in_bottom_left_direction_diagonal,
                    )
                };

                // Corners first (including diagonal extensions)
                if CORNER
                    && ((near_top && near_left)
                        || in_top_left_direction_diagonal)
                {
                    return R::top_left_direction();
                }
                if CORNER
                    && ((near_top && near_right)
                        || in_top_right_direction_diagonal)
                {
                    return R::top_right_direction();
                }
                if CORNER
                    && ((near_bottom && near_right)
                        || in_bottom_right_direction_diagonal)
                {
                    return R::bottom_right_direction();
                }
                if CORNER
                    && ((near_bottom && near_left)
                        || in_bottom_left_direction_diagonal)
                {
                    return R::bottom_left_direction();
                }

                // Pure edges
                if EDGE && near_top && !near_left && !near_right {
                    return R::top_direction();
                }
                if EDGE && near_right && !near_top && !near_bottom {
                    return R::right_direction();
                }
                if EDGE && near_bottom && !near_left && !near_right {
                    return R::bottom_direction();
                }
                if EDGE && near_left && !near_top && !near_bottom {
                    return R::left_direction();
                }

                R::none_directional()
            }
        }

        impl<R: $crate::math::geometry::ShapeDirectionType, const CS: bool>
            GetShapeDirectionType<$t, R> for Rectangle<$t, CS>
        {
            fn get_edge_or_corner_type(
                &self,
                point: ($t, $t),
                margin: $t,
            ) -> R {
                Self::get_edge_position::<true, true, R>(self, point, margin)
            }

            fn get_edge_type(&self, point: ($t, $t), margin: $t) -> R {
                Self::get_edge_position::<true, false, R>(self, point, margin)
            }

            fn get_corner_type(&self, point: ($t, $t), margin: $t) -> R {
                Self::get_edge_position::<false, true, R>(self, point, margin)
            }
        }
    };
}
// TODO: Make these traits auto impl
impl_const_rectangle_for_type!(i8);
impl_const_rectangle_for_type!(i16);
impl_const_rectangle_for_type!(i32);
impl_const_rectangle_for_type!(i64);
impl_const_rectangle_for_type!(i128);
impl_const_rectangle_for_type!(isize);
impl_const_rectangle_for_type!(u8);
impl_const_rectangle_for_type!(u16);
impl_const_rectangle_for_type!(u32);
impl_const_rectangle_for_type!(u64);
impl_const_rectangle_for_type!(u128);
impl_const_rectangle_for_type!(usize);
impl_const_rectangle_for_type!(f32);
impl_const_rectangle_for_type!(f64);
impl_const_rectangle_for_type!(f16);
impl_const_rectangle_for_type!(f128);
