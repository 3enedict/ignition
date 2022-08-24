use std::num::NonZeroU32;

use log::error;
use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, ImageCopyBuffer, ImageCopyTexture,
    ImageDataLayout, LoadOp, Operations, Origin3d, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceError, SurfaceTexture, TextureAspect, TextureView,
    TextureViewDescriptor,
};
use winit::event_loop::ControlFlow;

use crate::{
    manifestation::{Image, Renderer, Screen, GPU},
    Engine,
};

pub struct ScreenEncoder {
    frame: SurfaceTexture,
    view: TextureView,

    encoder: CommandEncoder,
}

impl Engine<Screen> {
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
        self.renderer.queue().submit(command_buffer);

        encoder.frame.present();

        Ok(())
    }
}

impl Engine<GPU> {
    pub fn encoder(&mut self) -> Result<CommandEncoder, ()> {
        Ok(create_command_encoder(self))
    }
}

impl Engine<Image<'static>> {
    pub fn encoder(&mut self) -> Result<CommandEncoder, ()> {
        Ok(create_command_encoder(self))
    }

    pub fn render_pass<'a>(&'a mut self, encoder: &'a mut CommandEncoder) -> RenderPass {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.renderer.view,
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

    pub fn render(&mut self, mut encoder: CommandEncoder, name: &str) -> Result<(), ()> {
        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                aspect: TextureAspect::All,
                texture: &self.renderer.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            ImageCopyBuffer {
                buffer: &self.renderer.buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(
                        std::mem::size_of::<u32>() as u32 * self.renderer.size,
                    ),
                    rows_per_image: NonZeroU32::new(self.renderer.size),
                },
            },
            self.renderer.description.size,
        );

        self.renderer.queue().submit(Some(encoder.finish()));

        {
            let buffer_slice = self.renderer.buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });

            self.renderer.gpu.device.poll(wgpu::Maintain::Wait);
            pollster::block_on(rx.receive()).unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(self.renderer.size, self.renderer.size, data)
                    .unwrap();
            buffer.save(name).unwrap();
        }

        self.renderer.buffer.unmap();
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

pub fn create_command_encoder<R: Renderer>(engine: &mut Engine<R>) -> CommandEncoder {
    let descriptor = &CommandEncoderDescriptor { label: None };
    engine.renderer.device().create_command_encoder(descriptor)
}
