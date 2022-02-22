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

pub fn create_command_buffer(engine: &Engine, view: &TextureView) -> Option<CommandBuffer> {
    let mut encoder = create_command_encoder(engine);

    begin_render_pass(&mut encoder, view, engine);

    Some(encoder.finish())
}

fn create_command_encoder(engine: &Engine) -> CommandEncoder {
    engine.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
        label: None,
    })
}

fn begin_render_pass(encoder: &mut CommandEncoder, view: &TextureView, engine: &Engine) {
    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
    });

    render_pass.set_pipeline(&engine.pipelines[0]);
    render_pass.draw(0..3, 0..1);
}
