use cgdraw_core::keyboard::keys::VirtualKeyCode;

pub enum AppEvent {
    Setup,
    Finished,
    Update,
    Draw,

    KeyPressed { key_code: VirtualKeyCode },
    KeyUp { key_code: VirtualKeyCode },

    Resize { width: u32, height: u32 },
}
