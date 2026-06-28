//! This lib is all about geometry (solid) and expressions (dynamic) shapes in 1, 2, 3, and maybe 4 dimensions
#![feature(const_ops)]
#![feature(const_trait_impl)]
// #![feature(const_trait_impl)]

// mod math;
// pub use math::*;
/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;

/// A position within a string
pub mod text_position;
