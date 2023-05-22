pub enum AppEvent {
    Setup,
    Finished,
    Update,
    Draw,
    Keydown,
    Keypress,
    Keyup,
    Resize { width: u32, height: u32 },
}
