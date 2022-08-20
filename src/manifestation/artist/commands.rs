use log::error;
use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, LoadOp, Operations, RenderPass,
    RenderPassDescriptor, SurfaceError, SurfaceTexture, TextureView, TextureViewDescriptor,
};

use winit::event_loop::ControlFlow;

use crate::{
    manifestation::{Renderer, Screen},
    Engine,
};

pub struct Commands {
    frame: SurfaceTexture,
    view: TextureView,

    encoder: CommandEncoder,
}

impl Commands {
    pub fn ignite(engine: &mut Engine<Screen>) -> Result<Self, ()> {
        let frame = create_frame(engine)?;
        let view = create_view(&frame);

        let encoder = create_command_encoder(engine);

        Ok(Self {
            frame,
            view,

            encoder,
        })
    }

    pub fn ignite_render_pass(&mut self) -> RenderPass {
        self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
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

    pub fn execute(self, engine: &mut Engine<Screen>) -> Result<(), ()> {
        let command_buffer = Some(self.encoder.finish());
        engine.renderer.queue().submit(command_buffer);

        self.frame.present();

        Ok(())
    }
}

pub fn create_frame(engine: &mut Engine<Screen>) -> Result<SurfaceTexture, ()> {
    match engine.renderer.surface.get_current_texture() {
        Ok(frame) => Ok(frame),
        Err(SurfaceError::Lost) => {
            engine.resize(engine.renderer.size);
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

pub fn create_command_encoder(engine: &mut Engine<Screen>) -> CommandEncoder {
    let descriptor = &CommandEncoderDescriptor { label: None };
    engine.renderer.device().create_command_encoder(descriptor)
}
