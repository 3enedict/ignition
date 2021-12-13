use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::swapchain as vulkano_swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::device::DeviceExtensions;
use vulkano::pipeline::viewport::Viewport;
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};

use crate::renderer::VglRenderer;

pub mod instance;
use instance::VglInstance;

pub mod surface;
use surface::VglSurface;

pub mod physical_device;
use physical_device::VglPhysicalDevice;

pub mod logical_device;
use logical_device::VglLogicalDevice;

pub mod swapchain;
use swapchain::VglSwapchain;

pub mod render_pass;
use render_pass::VglRenderPass;

pub mod pipeline;
use pipeline::VglPipeline;

pub mod framebuffers;
use framebuffers::VglFramebuffers;


#[derive(Default, Debug, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
    #version 450
    #extension GL_ARB_separate_shader_objects : enable

    out gl_PerVertex {
        vec4 gl_Position;
    };

    layout(location = 0) in vec2 position;

    layout(location = 0) out vec3 fragColor;

    vec3 colors[3] = vec3[](
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 1.0)
    );

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        fragColor = colors[gl_VertexIndex];
    }
      "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
    #version 450
    #extension GL_ARB_separate_shader_objects : enable

    layout(location = 0) in vec3 fragColor;

    layout(location = 0) out vec4 outColor;

    void main() {
        outColor = vec4(fragColor, 1.0);
    }
            "
    }
}

impl VglRenderer {
    pub fn new() -> Self {
        let instance = VglInstance::new();

        let event_loop = EventLoop::new();

        let surface = VglSurface::new(
            &instance,
            &event_loop,
        );

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let physical_device = VglPhysicalDevice::new(
            &instance,
            &surface,
            &device_extensions,
        );

        let logical_device = VglLogicalDevice::new(
            &device_extensions,
            &physical_device,
        );

        let swapchain = VglSwapchain::new(
            &surface,
            &physical_device,
            &logical_device,
        );

        vulkano::impl_vertex!(Vertex, position);

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::all(),
            false,
            [
            Vertex {
                position: [0.0, -0.5],
            },
            Vertex {
                position: [0.5, 0.5],
            },
            Vertex {
                position: [-0.5, 0.5],
            },
            ]
            .iter()
            .cloned(),
        )
            .unwrap();

        let vs = vs::Shader::load(logical_device.clone_logical_device()).unwrap();
        let fs = fs::Shader::load(logical_device.clone_logical_device()).unwrap();

        let render_pass = VglRenderPass::new(
            &logical_device,
            &swapchain,
        );

        let pipeline = VglPipeline::new(
            &logical_device,
            &render_pass,
            &vs,
            &fs,
        );

        // Dynamic viewports allow us to recreate just the viewport when the window is resized
        // Otherwise we would have to recreate the whole pipeline.
        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        // The render pass we created above only describes the layout of our framebuffers. Before we
        // can draw we also need to create the actual framebuffers.
        //
        // Since we need to draw to multiple images, we are going to create a different framebuffer for
        // each image.
        let framebuffers = VglFramebuffers::new(
            &swapchain,
            &render_pass,
            &mut viewport,
        );

        let previous_frame_end = Some(sync::now(logical_device.clone_logical_device()).boxed());

        Self {
            event_loop,
            surface,

            logical_device,

            swapchain,

            vertex_buffer,

            render_pass,

            pipeline,

            viewport,
            framebuffers,

            previous_frame_end,

            recreate_swapchain: false,
        }
    }


    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    self.previous_frame_end.as_mut().unwrap().cleanup_finished();

                    if self.recreate_swapchain {
                        if self.swapchain.recreate_swapchain(&self.surface) {
                            return;
                        }

                        self.framebuffers.recreate_framebuffers(
                            &self.swapchain,
                            &self.render_pass,
                            &mut self.viewport,
                        );

                        self.recreate_swapchain = false;
                    }

                    let (image_num, suboptimal, acquire_future) =
                        match vulkano_swapchain::acquire_next_image(self.swapchain.clone_swapchain(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                self.recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };

                    if suboptimal {
                        self.recreate_swapchain = true;
                    }

                    let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

                    // In order to draw, we have to build a *command buffer*. The command buffer object holds
                    // the list of commands that are going to be executed.
                    //
                    // Building a command buffer is an expensive operation (usually a few hundred
                    // microseconds), but it is known to be a hot path in the driver and is expected to be
                    // optimized.
                    //
                    // Note that we have to pass a queue family when we create the command buffer. The command
                    // buffer will only be executable on that given queue family.
                    let mut builder = AutoCommandBufferBuilder::primary(
                        self.logical_device.clone_logical_device(),
                        self.logical_device.get_queue().family(),
                        CommandBufferUsage::OneTimeSubmit,
                    )
                        .unwrap();

                    builder
                        // Before we can draw, we have to *enter a render pass*. There are two methods to do
                        // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
                        // not covered here.
                        //
                        // The third parameter builds the list of values to clear the attachments with. The API
                        // is similar to the list of attachments when building the framebuffers, except that
                        // only the attachments that use `load: Clear` appear in the list.
                        .begin_render_pass(
                            self.framebuffers.get_framebuffers()[image_num].clone(),
                            SubpassContents::Inline,
                            clear_values,
                        )
                        .unwrap()
                        // We are now inside the first subpass of the render pass. We add a draw command.
                        //
                        // The last two parameters contain the list of resources to pass to the shaders.
                        // Since we used an `EmptyPipeline` object, the objects have to be `()`.
                        .set_viewport(0, [self.viewport.clone()])
                        .bind_pipeline_graphics(self.pipeline.clone_pipeline())
                        .bind_vertex_buffers(0, self.vertex_buffer.clone())
                        .draw(self.vertex_buffer.len() as u32, 1, 0, 0)
                        .unwrap()
                        // We leave the render pass by calling `draw_end`. Note that if we had multiple
                        // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
                        // next subpass.
                        .end_render_pass()
                        .unwrap();

                    // Finish building the command buffer by calling `build`.
                    let command_buffer = builder.build().unwrap();

                    let future = self.previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.logical_device.clone_queue(), command_buffer)
                        .unwrap()
                        // The color output is now expected to contain our triangle. But in order to show it on
                        // the screen, we have to *present* the image by calling `present`.
                        //
                        // This function does not actually present the image immediately. Instead it submits a
                        // present command at the end of the queue. This means that it will only be presented once
                        // the GPU has finished executing the command buffer that draws the triangle.
                        .then_swapchain_present(self.logical_device.clone_queue(), self.swapchain.clone_swapchain(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            self.previous_frame_end = Some(future.boxed());
                        }
                        Err(FlushError::OutOfDate) => {
                            self.recreate_swapchain = true;
                            self.previous_frame_end = Some(sync::now(self.logical_device.clone_logical_device()).boxed());
                        }
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            self.previous_frame_end = Some(sync::now(self.logical_device.clone_logical_device()).boxed());
                        }
                        }
                }
                _ => (),
            }
        });
    }
}
