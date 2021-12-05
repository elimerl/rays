use glam::Vec3;

pub struct Img {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec3>,
}
impl Img {
    pub fn new(width: usize, height: usize) -> Img {
        Img {
            width,
            height,
            pixels: vec![Vec3::ZERO; (width * height) as usize],
        }
    }
    pub fn save(&self, path: &str) -> image::ImageResult<()> {
        image::RgbImage::from_raw(
            self.width as u32,
            self.height as u32,
            self.pixels
                .iter()
                .flat_map(|p| {
                    [
                        (p.x * 255.0) as u8,
                        (p.y * 255.0) as u8,
                        (p.z * 255.0) as u8,
                    ]
                })
                .collect(),
        )
        .unwrap()
        .save(path)?;
        Ok(())
    }
}
