use std::borrow::Cow;
use wgpu::{
    Instance,
    Backends,

    SurfaceError,
    TextureViewDescriptor,
};


use crate::core::Engine;

pub mod window;
use window::{IgnitionWindow, create_window, create_surface, generate_default_configuration};

pub mod gpu;
use gpu::{IgnitionGPU, get_adapter, get_device};

pub mod commands;
use commands::create_command_buffer;

impl Engine {
    pub async fn setup_engine() -> Self {
        env_logger::init();


        let (event_loop, window, size) = create_window();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState { 
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            window: IgnitionWindow {
                event_loop: Some(event_loop),
                window,
                size,

                surface,
                config,
            },

            gpu: IgnitionGPU {
                adapter,

                device,
                queue,
            },

            render_pipeline,
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let frame = self.window.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let command_buffer = create_command_buffer(self, &view);

        self.gpu.queue.submit(command_buffer);
        frame.present();

        Ok(())
    }
}
