use crate::{projection::CameraProjection, state::CameraState};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub view_position: [f32; 4],
}

impl CameraUniform {
    pub fn update_view_proj(&mut self, state: &CameraState, projection: &CameraProjection) {
        self.view_position = state.position.to_homogeneous().into();
        self.view_proj = (projection.to_wgpu_matrix() * state.calc_matrix()).into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}
