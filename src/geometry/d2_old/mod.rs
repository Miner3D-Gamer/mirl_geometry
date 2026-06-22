/// Circular collision math
pub mod circle;
/// Rectangular collision math
pub mod rectangle;

pub use circle::Circle;
pub use rectangle::Rectangle;

#[deprecated = "Please use `mirl::math::geometry::BOTTOM_HIGHER` instead"]
/// When the bottom of the collision is mathematically higher
pub const BOTTOM_HIGHER: bool = true;
#[deprecated = "Please use `mirl::math::geometry::BOTTOM_LOWER` instead"]
/// When the top of the collision is mathematically higher
pub const BOTTOM_LOWER: bool = false;

// /// Check if a circle and a rectangle collide
// pub fn do_circle_and_rectangle_collide<
//     T: NumberWithMonotoneOps + Copy,
//     const CS: bool,
// >(
//     rectangle: Rectangle<T, CS>,
//     circle: Circle<T, CS>,
// ) -> Option<(T, T)> {
//     let closest_x = clamp(circle.x, rectangle.left(), rectangle.right());
//     let closest_y = clamp(circle.y, rectangle.top(), rectangle.bottom());

//     let dx = circle.x - closest_x;
//     let dy = circle.y - closest_y;

//     if (dx * dx + dy * dy) <= (circle.radius * circle.radius) {
//         Some((closest_x, closest_y))
//     } else {
//         None
//     }
// }
