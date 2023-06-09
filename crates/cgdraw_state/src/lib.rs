use cgdraw_camera::uniform::CameraUniformFloat32;
use cgdraw_core::graphic::Texture;
use pipeline::MainPipeline;
use winit::window::Window;

mod pipeline;

pub struct State {
    /// É o dispositivo que permite criar recursos como buffers e texturas.
    pub device: wgpu::Device,

    /// É a fila de comandos que permite enviar comandos para o dispositivo.
    pub queue: wgpu::Queue,

    /// É a superfície que permite renderizar os gráficos.
    pub surface: wgpu::Surface,

    /// É a configuração da superfície.
    pub surface_config: wgpu::SurfaceConfiguration,

    /// O uniform da câmera que será usado para enviar os dados da câmera para o shader.
    pub camera_uniform: CameraUniformFloat32,

    /// O buffer da câmera que será usado para enviar os dados da câmera para o shader.
    pub camera_buffer: wgpu::Buffer,

    /// O layout do grupo de ligação da câmera que será usado para criar o grupo de ligação da câmera.
    pub camera_bind_group_layout: wgpu::BindGroupLayout,

    /// O grupo de ligação da câmera que será usado para enviar os dados da câmera para o shader.
    pub camera_bind_group: wgpu::BindGroup,

    /// O pipeline principal que será usado para renderizar os gráficos.
    pub main_pipeline: MainPipeline,

    /// A textura de profundidade que será usada para renderizar os gráficos.
    pub depth_view: wgpu::TextureView,
}

impl State {
    pub async fn new(window: &Window, camera_uniform: CameraUniformFloat32) -> Self {
        let size = window.inner_size();

        // Backends: Vulkan, Metal, DX12, DX11, Browser WebGPU e GL
        // Usado para criar o dispositivo e a fila de comandos.
        let backends = wgpu::Backends::all();

        let dx12_shader_compiler = Default::default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler,
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
                    } else {
                        wgpu::Limits::default().using_resolution(adapter.limits())
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let camera_buffer = CameraUniformFloat32::create_buffer(&device, camera_uniform);
        let camera_bind_group_layout = CameraUniformFloat32::create_bind_group_layout(&device);
        let camera_bind_group = CameraUniformFloat32::create_bind_group(
            &device,
            &camera_bind_group_layout,
            &camera_buffer,
        );

        let main_pipeline =
            MainPipeline::new(&device, surface_config.format, &[&camera_bind_group_layout]);

        let depth_view = Texture::create_depth_texture(&device, &surface_config).view;

        Self {
            device,
            queue,
            surface,
            surface_config,
            camera_uniform,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
            main_pipeline,
            depth_view,
        }
    }
}
