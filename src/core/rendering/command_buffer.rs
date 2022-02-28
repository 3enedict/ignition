use wgpu::{
    CommandEncoder,
    RenderPass,
    TextureView,
    SurfaceTexture,

    CommandEncoderDescriptor,
    RenderPassDescriptor,
    TextureViewDescriptor,

    LoadOp,
    Operations,
    Color,
};

use crate::core::Engine;

#[macro_export]
macro_rules! draw {
    ( $( $x:expr ),* ) => {
        $(
            engine.queue_draw($x);
        )*
    };
}


pub fn create_frame(engine: &Engine) -> (SurfaceTexture, TextureView) {
    let frame = engine.window.surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    let view = frame.texture.create_view(&TextureViewDescriptor::default());

    (frame, view)
}

pub fn create_command_encoder(engine: &Engine) -> CommandEncoder {
    engine.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
        label: None,
    })
}

pub fn create_render_pass<'a>(encoder: &'a mut CommandEncoder, view: &'a TextureView) -> RenderPass<'a> {
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
