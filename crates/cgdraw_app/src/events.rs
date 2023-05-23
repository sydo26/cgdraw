use cgdraw_core::keyboard::keys::VirtualKeyCode;

use crate::graphics::Graphics;

pub enum AppEvent<'a> {
    Setup,
    Finished,
    Update,
    Draw { graphics: &'a mut Graphics<'a> },

    KeyPressed { key_code: VirtualKeyCode },
    KeyUp { key_code: VirtualKeyCode },

    Resize { width: u32, height: u32 },
}
