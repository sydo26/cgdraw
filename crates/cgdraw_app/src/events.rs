use cgdraw_core::keyboard::keys::VirtualKeyCode;

pub enum AppEvent {
    Setup,
    Finished,
    Update,
    Draw,
    KeyDown,
    KeyPressed { key_code: VirtualKeyCode },
    Keyup,
    Resize { width: u32, height: u32 },
}
