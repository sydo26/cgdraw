pub struct WindowPhysicalSize {
    pub width: u32,
    pub height: u32,
}

impl WindowPhysicalSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
