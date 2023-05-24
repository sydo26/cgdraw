pub use cgdraw_app::camera::CameraInitialAttributes;
pub use cgdraw_app::App;

pub use cgdraw_core::color::Color;
pub use cgdraw_core::keyboard::keys::VirtualKeyCode;

pub use cgdraw_camera::Camera;
pub use cgdraw_core::projection::Projection;

pub mod consts {
    pub use cgdraw_core::projection::OPENGL_TO_WGPU_MATRIX;
}

pub mod graphics {
    pub use cgdraw_core::graphic::Vertex;
}

pub mod event {
    pub use cgdraw_app::events::AppEvent;
}
