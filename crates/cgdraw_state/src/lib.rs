use winit::window::Window;

pub struct State {
    /**
     * É o dispositivo que permite criar recursos como buffers e texturas.
     */
    pub device: wgpu::Device,

    /**
     * É a fila de comandos que permite enviar comandos para o dispositivo.
     */
    pub queue: wgpu::Queue,

    /**
     * É a superfície que permite renderizar os gráficos.
     */
    pub surface: wgpu::Surface,

    /**
     * É a configuração da superfície.
     */
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Backends: Vulkan, Metal, DX12, DX11, Browser WebGPU e GL
        // Usado para criar o dispositivo e a fila de comandos.
        let backends = wgpu::Backends::all();

        // let dx12_shader_compiler = wgpu::Dx12Compiler::Dxc {
        //     dxil_path: None,
        //     dxc_path: None,
        // };
        let dx12_shader_compiler = Default::default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler,
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        // let adapter = instance
        //     .enumerate_adapters(backends)
        //     .filter(|adapter| {
        //         // Verifica se o adaptador suporta a superfície usada.
        //         adapter.is_surface_supported(&surface)
        //     })
        //     .next()
        //     .unwrap();
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

        Self {
            device,
            queue,
            surface,
            surface_config,
        }
    }
}
