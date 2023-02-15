mod dielectric;
mod lambertian;
mod material;
mod metallic;

use cgmath::Vector3;

pub mod prelude {
    pub use super::dielectric::Dielectric;
    pub use super::lambertian::Lambertian;
    pub use super::material::{Material, MaterialTrait, ScatterResult};
    pub use super::metallic::Metallic;
}

pub type Color = Vector3<f64>;
