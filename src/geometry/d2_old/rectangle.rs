// impl<
//         T: core::ops::Add<Output = T>
//             + core::ops::Div<Output = T>
//             + core::marker::Copy
//             + core::cmp::PartialOrd,
//         const CS: bool,
//     > From<(T, T, T, T)> for Rectangle<T, CS>
// {
//     fn from(tuple: (T, T, T, T)) -> Self {
//         Self::new((tuple.0, tuple.1), (tuple.2, tuple.3))
//     }
// }
// impl<T, const CS: bool> std::fmt::Display for Rectangle<T, CS> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }
// TODO: Add support for non Pos2D positioning -> Tuple input
use crate::math::geometry::{positioning::*, GetShapeDirectionType};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A simple Rectangle defining computational limits
#[allow(missing_docs)]
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub struct Rectangle<T, const CS: bool> {
    pub width: T,
    pub height: T,
}
impl<T, const CS: bool> Shape<T> for Rectangle<T, CS> {}

impl<T: Default, const CS: bool> Default for Rectangle<T, CS> {
    fn default() -> Self {
        Self {
            width: T::default(),
            height: T::default(),
        }
    }
}
impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: Copy,
{
    #[must_use]
    /// Create a new Rectangle
    pub const fn new(size: (T, T)) -> Self {
        Self {
            width: size.0,
            height: size.1,
        }
    }
    // /// Set the current position of the rectangle
    // pub const fn set_pos(&mut self, pos: (T, T)) {
    //     self.x = pos.0;    //     self.y = pos.1;    // }
    /// Set the current size of the rectangle
    pub const fn set_size(&mut self, size: (T, T)) {
        self.width = size.0;
        self.height = size.1;
    }
}
impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: Copy,
{
    #[must_use]
    #[inline(always)]
    /// Get the current size
    pub const fn get_size(&self) -> (T, T) {
        (self.width, self.height)
    }
    /// Get the current width
    #[must_use]
    #[inline(always)]
    pub const fn get_width(&self) -> T {
        self.width
    }
    ///Get the current height
    #[must_use]
    #[inline(always)]
    pub const fn get_height(&self) -> T {
        self.height
    }
}
impl<T, const BOTTOM_HIGHER: bool> Pos2D<T, Rectangle<T, BOTTOM_HIGHER>>
where
    T: Copy,
{
    #[must_use]
    #[inline(always)]
    /// Get the current size
    pub const fn get_size(&self) -> (T, T) {
        self.shape.get_size()
    }
    /// Get the current width
    #[must_use]
    #[inline(always)]
    pub const fn get_width(&self) -> T {
        self.shape.get_width()
    }
    ///Get the current height
    #[must_use]
    #[inline(always)]
    pub const fn get_height(&self) -> T {
        self.shape.get_height()
    }
}
impl<T, const BOTTOM_HIGHER: bool> Pos2D<T, Rectangle<T, BOTTOM_HIGHER>>
where
    T: core::ops::Add<Output = T> + PartialOrd + Copy,
{
    /// Get the x of the left side of this rectangle
    #[must_use]
    #[inline(always)]
    pub fn offset_pos(&self, offset_pos: (T, T)) -> Self {
        let x = self.get_x();
        let y = self.get_y();
        Self {
            pos: (x + offset_pos.0, offset_pos.1 + y),
            shape: Rectangle {
                width: self.get_width(),
                height: self.get_height(),
            },
        }
    }
    /// Get the x of the left side of this rectangle
    #[must_use]
    #[inline(always)]
    pub const fn left(&self) -> T {
        self.get_x()
    }
    /// Get the x of the right side of this rectangle
    #[must_use]
    #[inline(always)]
    pub const fn right(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        self.get_x() + self.get_width()
    }
    /// Get the y of the top side of this rectangle
    #[must_use]
    #[inline(always)]
    pub const fn top(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.get_y() + self.get_height()
        } else {
            self.get_y()
        }
    }
    #[must_use]
    /// Get the y of the bottom side of this rectangle
    #[inline(always)]
    pub const fn bottom(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.get_y()
        } else {
            self.get_y() + self.get_height()
        }
    }
}
impl<V, const BOTTOM_HIGHER: bool> Pos2D<V, Rectangle<V, BOTTOM_HIGHER>>
where
    V: core::ops::Div<Output = V> + PartialOrd + Copy,
{
    /// Get the value you'd need to divide width by to get height
    #[must_use]
    #[inline(always)]
    pub fn get_ratio(&self) -> V {
        self.shape.get_ratio()
    }
}
impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: core::ops::Div<Output = T> + PartialOrd + Copy,
{
    /// Get the value you'd need to divide width by to get height
    #[must_use]
    #[inline(always)]
    pub fn get_ratio(&self) -> T {
        self.height / self.width
    }
    // #[must_use]
    // /// Checks if a point falls within the coordinate range defined by the triangle bounds
    // pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
    //     if BOTTOM_HIGHER {
    //         point.0 >= self.left()
    //             && point.0 <= self.right()
    //             && point.1 >= self.bottom()
    //             && point.1 <= self.top()
    //     } else {
    //         point.0 >= self.left()
    //             && point.0 <= self.right()
    //             && point.1 >= self.top()
    //             && point.1 <= self.bottom()
    //     }
    // }
}
use mirl_extensions::*;

use crate::math::geometry::{Pos2D, Shape};
impl<
        T: ConstNumbers128
            + core::ops::Add<Output = T>
            + core::ops::Div<Output = T>
            + Copy,
        const CS: bool,
    > Pos2D<T, Rectangle<T, CS>>
{
    #[must_use]
    #[inline(always)]
    /// Get the center position of the rectangle
    pub fn center(&self) -> (T, T) {
        (
            self.get_x() + self.shape.get_width() / T::CONST_2,
            self.get_y() + self.shape.get_height() / T::CONST_2,
        )
    }
}

#[macro_export]
/// Automatically implement a bunch of rectangle math for the given type
macro_rules! impl_const_rectangle_for_type {
    ($t:ty) => {
        impl<const CS: bool> Pos2D<$t, Rectangle<$t, CS>> {
            #[must_use]
            #[inline(always)]
            /// Checks if 2 rectangles collide anywhere
            pub const fn do_areas_intersect(
                &self,
                other: &Pos2D<$t, Rectangle<$t, CS>>,
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
            #[must_use]
            #[inline(always)]
            /// Checks if 2 rectangles collide anywhere
            pub const fn do_areas_intersect_strict(
                &self,
                other: &Pos2D<$t, Rectangle<$t, CS>>,
            ) -> bool {
                self.left() < other.right()
                    && self.right() > other.left()
                    && if CS {
                        self.bottom() < other.top()
                            && self.top() > other.bottom()
                    } else {
                        self.top() <= other.bottom()
                            && self.bottom() > other.top()
                    }
            }

            #[must_use]
            #[inline]
            /// Checks if this area fully includes another area
            pub const fn does_area_fully_include_other_area(
                &self,
                other: &Pos2D<$t, Rectangle<$t, CS>>,
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

            #[must_use]
            #[inline(always)]
            /// Checks if a point falls within the coordinate range defined by the rectangle bounds
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
            #[must_use]
            #[inline(always)]
            /// Check if point is within margin distance of any edge
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
            #[must_use]
            #[allow(clippy::too_many_lines)] // Jokes on you clippy, I am NOT touching that as long as it works
            /// Returns which edge/corner a point is near
            /// 0: top-left, 1: top, 2: top-right, 3: right, 4: bottom-right, 5: bottom, 6: bottom-left, 7: left
            /// Returns `u8::MAX` if not within margin of any edge
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
                // Extended corner regions - check diagonal extensions beyond the rectangle
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
                // Check corners first (including diagonal extensions)
                if CORNER
                    && ((near_top && near_left)
                        || in_top_left_direction_diagonal)
                {
                    return R::top_left_direction(); // top-left
                }
                if CORNER
                    && ((near_top && near_right)
                        || in_top_right_direction_diagonal)
                {
                    return R::top_right_direction(); // top-right
                }
                if CORNER
                    && ((near_bottom && near_right)
                        || in_bottom_right_direction_diagonal)
                {
                    return R::bottom_right_direction(); // bottom-right
                }
                if CORNER
                    && ((near_bottom && near_left)
                        || in_bottom_left_direction_diagonal)
                {
                    return R::bottom_left_direction(); // bottom-left
                }

                // Check edges
                if EDGE && near_top && !near_left && !near_right {
                    return R::top_direction(); // top
                }
                if EDGE && near_right && !near_top && !near_bottom {
                    return R::right_direction(); // right
                }
                if EDGE && near_bottom && !near_left && !near_right {
                    return R::bottom_direction(); // bottom
                }
                if EDGE && near_left && !near_top && !near_bottom {
                    return R::left_direction(); // left
                }

                // Not near any edge
                R::none_directional()
            }
        }
        impl<R: $crate::math::geometry::ShapeDirectionType, const CS: bool>
            GetShapeDirectionType<$t, R> for Pos2D<$t, Rectangle<$t, CS>>
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
