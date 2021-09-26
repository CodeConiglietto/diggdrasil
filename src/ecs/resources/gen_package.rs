use noise::Perlin;

#[derive(Default)]
pub struct GenPackageResource {
    pub elevation_noise: Perlin,
    pub fertility_noise: Perlin,
}
