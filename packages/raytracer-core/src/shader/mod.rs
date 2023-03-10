mod dielectric;
mod lambertian;
mod material;
mod metallic;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, MaterialTrait};
pub use metallic::Metallic;
