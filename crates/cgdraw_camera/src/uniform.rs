use cgdraw_math::matrix::Matrix4x4;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformFloat32 {
    pub view_proj: [[f32; 4]; 4],
    pub view_position: [f32; 4],
}

impl Default for CameraUniformFloat32 {
    fn default() -> CameraUniformFloat32 {
        CameraUniformFloat32 {
            view_proj: Matrix4x4::identity().into(),
            view_position: [0.0; 4],
        }
    }
}

macro_rules! impl_camera_uniform {
    ($CameraUniformN:ty) => {
        impl $CameraUniformN {
            /// Cria um buffer da câmera para enviar para o shader.
            pub fn create_buffer(device: &wgpu::Device, uniform: Self) -> wgpu::Buffer {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                })
            }

            /// Cria um `bind_group_layout` para a câmera, que será usado para criar um `bind_group`.
            pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                })
            }

            /// Cria um `bind_group` para a câmera, que será usado pelo `render_pipeline`.
            pub fn create_bind_group(
                device: &wgpu::Device,
                layout: &wgpu::BindGroupLayout,
                buffer: &wgpu::Buffer,
            ) -> wgpu::BindGroup {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                    label: Some("camera_bind_group"),
                })
            }
        }
    };
}

impl_camera_uniform!(CameraUniformFloat32);
