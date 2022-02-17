use std::iter::once;

use wgpu::{
    Instance,
    Backends,

    SurfaceError,
    TextureViewDescriptor,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    LoadOp,
    Operations,
    Color,
};


use crate::core::Engine;

pub mod window;
use window::IgnitionWindow;

pub mod gpu;
use gpu::IgnitionGPU;

impl Engine {
    pub async fn setup_engine() -> Self {
        env_logger::init();

        let instance = Instance::new(Backends::all());

        let mut window = IgnitionWindow::new(&instance);
        let gpu = pollster::block_on(IgnitionGPU::new(&instance, &window));

        window.config.format = window.surface.get_preferred_format(&gpu.adapter).unwrap();

        window.surface
            .configure(&gpu.device, &window.config);

        Self {
            window,
            gpu,
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
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

        self.gpu.queue.submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}
