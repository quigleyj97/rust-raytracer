mod dielectric;
mod lambertian;
mod material;
mod metallic;

use cgmath::Vector3;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, MaterialTrait};
pub use metallic::Metallic;

pub type Color = Vector3<f64>;
