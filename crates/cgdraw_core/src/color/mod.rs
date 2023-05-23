use self::srgb::SrgbColor;

mod srgb;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    WHITE,
    BLACK,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    CYAN,
    MAGENTA,
    ORANGE,
    PINK,
    GRAY,
    GOLD,
    SILVER,
    LIME,
    MAROON,
    PURPLE,
    VIOLET,
    BEIGE,
    BROWN,
    DARKBLUE,
    DARKGREEN,
    DARKPURPLE,
    DARKBROWN,
    LIGHTGRAY,
    DARKGRAY,
    SKYBLUE,
}

impl Color {
    pub fn srgb(r: u8, g: u8, b: u8) -> SrgbColor {
        SrgbColor::from_raw(r, g, b)
    }
}

impl Color {
    pub fn get_srgb(self) -> SrgbColor {
        match self {
            Color::WHITE => SrgbColor::from_raw(255, 255, 255),
            Color::BLACK => SrgbColor::from_raw(0, 0, 0),
            Color::RED => SrgbColor::from_raw(255, 0, 0),
            Color::GREEN => SrgbColor::from_raw(0, 255, 0),
            Color::BLUE => SrgbColor::from_raw(0, 0, 255),
            Color::YELLOW => SrgbColor::from_raw(255, 255, 0),
            Color::CYAN => SrgbColor::from_raw(0, 255, 255),
            Color::MAGENTA => SrgbColor::from_raw(255, 0, 255),
            Color::ORANGE => SrgbColor::from_raw(255, 128, 0),
            Color::PINK => SrgbColor::from_raw(255, 192, 203),
            Color::GRAY => SrgbColor::from_raw(128, 128, 128),
            Color::GOLD => SrgbColor::from_raw(255, 215, 0),
            Color::SILVER => SrgbColor::from_raw(192, 192, 192),
            Color::LIME => SrgbColor::from_raw(0, 255, 0),
            Color::MAROON => SrgbColor::from_raw(128, 0, 0),
            Color::PURPLE => SrgbColor::from_raw(128, 0, 128),
            Color::VIOLET => SrgbColor::from_raw(128, 0, 128),
            Color::BEIGE => SrgbColor::from_raw(245, 245, 220),
            Color::BROWN => SrgbColor::from_raw(165, 42, 42),
            Color::DARKBLUE => SrgbColor::from_raw(0, 0, 139),
            Color::DARKGREEN => SrgbColor::from_raw(0, 100, 0),
            Color::DARKPURPLE => SrgbColor::from_raw(139, 0, 139),
            Color::DARKBROWN => SrgbColor::from_raw(101, 67, 33),
            Color::LIGHTGRAY => SrgbColor::from_raw(211, 211, 211),
            Color::DARKGRAY => SrgbColor::from_raw(169, 169, 169),
            Color::SKYBLUE => SrgbColor::from_raw(135, 206, 235),
        }
    }

    pub fn to_vector(self) -> [f32; 3] {
        let srgb = self.get_srgb();

        [srgb.r, srgb.g, srgb.b]
    }
}
