pub enum Material {
    Diffuse(glam::Vec3),
    Metal(glam::Vec3, f32),
    Dielectric(f32)
}