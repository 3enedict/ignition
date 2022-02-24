use wgpu::{
    CommandBuffer,
    CommandEncoder,
    RenderPass,
    TextureView,

    CommandEncoderDescriptor,
    RenderPassDescriptor,

    LoadOp,
    Operations,
    Color,
};

use crate::core::Engine;

pub fn create_command_buffer(engine: &mut Engine, view: &TextureView) -> Option<CommandBuffer> {
    let mut encoder = create_command_encoder(engine);
    setup_render_pass(engine, &mut encoder, view);

    Some(encoder.finish())
}

fn create_command_encoder(engine: &Engine) -> CommandEncoder {
    engine.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
        label: None,
    })
}

fn setup_render_pass(engine: &mut Engine, encoder: &mut CommandEncoder, view: &TextureView) {
    let mut render_pass = begin_render_pass(encoder, view);

    render_pass.set_pipeline(&engine.shapes.pipelines[0]);
    render_pass.set_vertex_buffer(0, engine.shapes.vertex_buffers[0].slice(..));

    render_pass.draw(0..engine.shapes.vertex_len[0], 0..1);
}

fn begin_render_pass<'a>(encoder: &'a mut CommandEncoder, view: &'a TextureView) -> RenderPass<'a> {
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
