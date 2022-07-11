use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, LoadOp, Operations, RenderPass,
    RenderPassDescriptor, SurfaceError, SurfaceTexture, TextureView, TextureViewDescriptor,
};

use crate::Engine;

pub struct Commands {
    frame: SurfaceTexture,
    view: TextureView,

    encoder: CommandEncoder,
}

impl Commands {
    pub fn ignite(engine: &Engine) -> Result<Self, SurfaceError> {
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
        create_render_pass(&mut self.encoder, &self.view)
    }

    pub fn execute(self, engine: &Engine) {
        let command_buffer = Some(self.encoder.finish());
        engine.renderer.queue.submit(command_buffer);

        self.frame.present();
    }
}

pub fn create_frame(engine: &Engine) -> Result<SurfaceTexture, SurfaceError> {
    let frame = engine
        .renderer
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    Ok(frame)
}

pub fn create_view(frame: &SurfaceTexture) -> TextureView {
    frame.texture.create_view(&TextureViewDescriptor::default())
}

pub fn create_command_encoder(engine: &Engine) -> CommandEncoder {
    engine
        .renderer
        .device
        .create_command_encoder(&CommandEncoderDescriptor { label: None })
}

pub fn create_render_pass<'a>(
    encoder: &'a mut CommandEncoder,
    view: &'a TextureView,
) -> RenderPass<'a> {
    encoder.begin_render_pass(&RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view,
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
    })
}
