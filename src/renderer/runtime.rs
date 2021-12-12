use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::swapchain as vulkano_swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::renderer::VglRenderer;

impl VglRenderer {
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
                    // It is important to call this function from time to time, otherwise resources will keep
                    // accumulating and you will eventually reach an out of memory error.
                    // Calling this function polls various fences in order to determine what the GPU has
                    // already processed, and frees the resources that are no longer needed.
                    self.previous_frame_end.as_mut().unwrap().cleanup_finished();

                    // Whenever the window resizes we need to recreate everything dependent on the window size.
                    // In this example that includes the swapchain, the framebuffers and the dynamic state viewport.
                    if self.recreate_swapchain {
                        if self.swapchain.recreate_swapchain(&self.surface) {
                            return;
                        }

                        // Because framebuffers contains an Arc on the old swapchain, we need to
                        // recreate framebuffers as well.
                        self.framebuffers.recreate_framebuffers(
                            &self.swapchain,
                            &self.render_pass,
                            &mut self.viewport,
                        );
                        self.recreate_swapchain = false;
                    }

                    // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
                    // no image is available (which happens if you submit draw commands too quickly), then the
                    // function will block.
                    // This operation returns the index of the image that we are allowed to draw upon.
                    //
                    // This function can block if no image is available. The parameter is an optional timeout
                    // after which the function call will return an error.
                    let (image_num, suboptimal, acquire_future) =
                        match vulkano_swapchain::acquire_next_image(self.swapchain.clone_swapchain(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                self.recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };

                    // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
                    // will still work, but it may not display correctly. With some drivers this can be when
                    // the window resizes, but it may not cause the swapchain to become out of date.
                    if suboptimal {
                        self.recreate_swapchain = true;
                    }

                    // Specify the color to clear the framebuffer with i.e. blue
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
