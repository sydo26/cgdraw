use cgdraw_math::matrix::Matrix4x4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformFloat32 {
    pub view_proj: [[f32; 4]; 4],
    pub view_position: [f32; 4],
}

impl Default for CameraUniformFloat32 {
    fn default() -> Self {
        Self {
            view_proj: Matrix4x4::identity().into(),
            view_position: [0.0; 4],
        }
    }
}
