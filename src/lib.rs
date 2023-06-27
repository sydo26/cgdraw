pub use cgdraw_app::builder::AppBuilder;
pub use cgdraw_app::App;

pub use cgdraw_core::color::Color;
pub use cgdraw_core::keyboard::keys::VirtualKeyCode;

pub use cgdraw_camera::Camera;
pub use cgdraw_camera::CameraAttributes;

pub mod consts {}

pub mod graphics {
    pub use cgdraw_app::graphics::Graphics;
    pub use cgdraw_core::graphic::Vertex;
}

pub mod event {
    pub use cgdraw_app::events::AppEvent;
}

pub mod math {
    pub use cgdraw_core::orthographic;
    pub use cgdraw_core::perspective;
    pub use cgdraw_math::matrix::*;
    pub use cgdraw_math::point::*;
    pub use cgdraw_math::vector::*;
}

pub mod angle {
    pub use cgdraw_math::angle::*;
}
