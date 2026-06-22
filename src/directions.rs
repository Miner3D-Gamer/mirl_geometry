pub use mirl_geometry_core::directions::*;

/// # A resizing helper function
///
/// Using the corner type from [`mirl::math::collision::rectangle::Rectangle::get_edge_position`](crate::math::collision::rectangle::Rectangle::get_edge_position) converts the given delta into a change of x, y, width, and height of a rectangle
#[must_use]
pub const fn corner_type_and_delta_to_metric_change<
    T: [const] core::ops::Neg<Output = T> + mirl_extensions::ConstZero + Copy,
>(
    corner: u8,
    mouse_pos_delta: (T, T),
) -> (T, T, T, T) {
    match corner {
        0 => (
            mouse_pos_delta.0,
            mouse_pos_delta.1,
            -mouse_pos_delta.0,
            -mouse_pos_delta.1,
        ),
        1 => (T::ZERO, mouse_pos_delta.1, T::ZERO, -mouse_pos_delta.1),
        2 => (
            T::ZERO,
            mouse_pos_delta.1,
            mouse_pos_delta.0,
            -mouse_pos_delta.1,
        ),
        3 => (T::ZERO, T::ZERO, mouse_pos_delta.0, T::ZERO),
        4 => (T::ZERO, T::ZERO, mouse_pos_delta.0, mouse_pos_delta.1),
        5 => (T::ZERO, T::ZERO, T::ZERO, mouse_pos_delta.1),
        6 => (
            mouse_pos_delta.0,
            T::ZERO,
            -mouse_pos_delta.0,
            mouse_pos_delta.1,
        ),
        7 => (mouse_pos_delta.0, T::ZERO, -mouse_pos_delta.0, T::ZERO),
        _ => (T::ZERO, T::ZERO, T::ZERO, T::ZERO),
    }
}
