use log::error;
use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, LoadOp, Operations, RenderPass,
    RenderPassColorAttachment, RenderPassDescriptor, SurfaceError, SurfaceTexture, TextureView,
    TextureViewDescriptor,
};
use winit::event_loop::ControlFlow;

use crate::Engine;

pub struct ScreenEncoder {
    frame: SurfaceTexture,
    view: TextureView,

    encoder: CommandEncoder,
}

impl Engine {
    pub fn encoder(&mut self) -> Result<ScreenEncoder, ()> {
        let frame = create_frame(self)?;
        let view = create_view(&frame);

        let encoder = create_command_encoder(self);

        Ok(ScreenEncoder {
            frame,
            view,

            encoder,
        })
    }

    pub fn render_pass<'a>(&'a mut self, encoder: &'a mut ScreenEncoder) -> RenderPass {
        encoder.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &encoder.view,
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
            })],
            depth_stencil_attachment: None,
        })
    }

    pub fn render(&mut self, encoder: ScreenEncoder) -> Result<(), ()> {
        let command_buffer = Some(encoder.encoder.finish());
        self.renderer.queue.submit(command_buffer);

        encoder.frame.present();

        Ok(())
    }
}

pub fn create_frame(engine: &mut Engine) -> Result<SurfaceTexture, ()> {
    match engine.renderer.surface.get_current_texture() {
        Ok(frame) => Ok(frame),
        Err(SurfaceError::Lost) => {
            engine.resize(engine.config.size);
            Err(())
        }
        Err(SurfaceError::OutOfMemory) => {
            error!("GPU out of memory - Ignition");
            engine.config.control_flow = ControlFlow::Exit;
            Err(())
        }
        Err(e) => {
            error!("{:?} - Ignition", e);
            Err(())
        }
    }
}

pub fn create_view(frame: &SurfaceTexture) -> TextureView {
    frame.texture.create_view(&TextureViewDescriptor::default())
}

pub fn create_command_encoder(engine: &mut Engine) -> CommandEncoder {
    let descriptor = &CommandEncoderDescriptor { label: None };
    engine.renderer.device.create_command_encoder(descriptor)
}
