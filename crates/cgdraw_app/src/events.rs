use std::time::Duration;

use cgdraw_core::keyboard::keys::VirtualKeyCode;

use crate::{graphics::Graphics, handler::AppHandler};

pub enum AppEvent<'a> {
    Setup,

    Finished,

    Update {
        handler: &'a mut AppHandler<'a>,
        delta_time: Duration,
    },

    Draw {
        graphics: &'a mut Graphics<'a>,
    },

    KeyPressed {
        key_code: VirtualKeyCode,
    },
    KeyUp {
        key_code: VirtualKeyCode,
    },

    Resize {
        width: u32,
        height: u32,
    },
}
