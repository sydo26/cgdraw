use cgdraw_math::matrix::Matrix4x4;

use self::camera::CameraUniformFloat32;

mod camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformsFloat32 {
    pub camera: CameraUniformFloat32,
    pub model: [[f32; 4]; 4],
}

impl Default for UniformsFloat32 {
    fn default() -> UniformsFloat32 {
        UniformsFloat32 {
            camera: CameraUniformFloat32::default(),
            model: Matrix4x4::identity().into(),
        }
    }
}

impl UniformsFloat32 {
    pub fn create_buffer(device: &wgpu::Device, uniform: Self) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniforms Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    /// Cria um `bind_group_layout` para o objeto uniforms, que será usado para criar um `bind_group`.
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
            label: Some("uniforms_bind_group_layout"),
        })
    }

    /// Cria um `bind_group` para o objeto uniforms, que será usado pelo `render_pipeline`.
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
            label: Some("uniforms_bind_group"),
        })
    }
}
