use std::iter::once;

use wgpu::{
    Instance,
    Backends,
    Surface,
    Device,
    Queue,

    RequestAdapterOptions,

    PowerPreference,
    DeviceDescriptor,
    Features,
    Limits,
    SurfaceConfiguration,
    SurfaceError,
    TextureUsages,
    PresentMode,
    TextureViewDescriptor,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    LoadOp,
    Operations,
    Color,
};

use winit::dpi::PhysicalSize;

mod window;
use window::create_window;
use window::VglWindow;

pub struct VglRenderer {
    pub window: VglWindow,

    surface: Surface,
    config: SurfaceConfiguration,

    device: Device,
    queue: Queue,
}

impl VglRenderer {
    pub async fn new() -> Self {
        env_logger::init();

        let window = create_window();

        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(&window.window) };
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let adapter_info = adapter.get_info();
        println!("Device name : {}", adapter_info.name);

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();


        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window.size.width,
            height: window.size.height,
            present_mode: PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Self {
            window,

            surface,
            device,
            queue,
            config,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.window.size = new_size;

            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn create_renderer() -> VglRenderer {
    pollster::block_on(VglRenderer::new())
}
