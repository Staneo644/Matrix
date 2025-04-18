pub mod linear_interpolation;
pub mod cosine;
pub mod cross_product;
pub mod linear_combination;

pub use linear_interpolation::lerp;
pub use linear_combination::linear_combination;
pub use cosine::angle_cos;
pub use cross_product::cross_product;