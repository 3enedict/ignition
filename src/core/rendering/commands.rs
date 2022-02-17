use std::iter::{Once, once};

use wgpu::{
    CommandBuffer,
    CommandEncoder,
    TextureView,

    CommandEncoderDescriptor,
    RenderPassDescriptor,

    LoadOp,
    Operations,
    Color,
};

use crate::core::Engine;

pub fn create_command_buffer(engine: &Engine, view: &TextureView) -> Once<CommandBuffer> {
    let mut encoder = create_command_encoder(engine);

    begin_render_pass(&mut encoder, view);

    once(encoder.finish())
}

fn create_command_encoder(engine: &Engine) -> CommandEncoder {
    engine.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    })
}

fn begin_render_pass(encoder: &mut CommandEncoder, view: &TextureView) {
    encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("Render Pass"),
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
    });
}
