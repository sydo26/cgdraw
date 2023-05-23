#[derive(Debug, Copy, Clone)]
pub struct SrgbColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl SrgbColor {
    pub fn from_raw(r: u8, g: u8, b: u8) -> SrgbColor {
        use palette::Srgb;

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let srgb = Srgb::new(r, g, b);

        Self {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
        }
    }
}
