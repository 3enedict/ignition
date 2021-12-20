use vulkano::device::DeviceExtensions;
use vulkano::pipeline::viewport::Viewport;
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


pub mod swapchain_image;
use swapchain_image::VglSwapchainImage;

pub mod command_buffer;
use command_buffer::VglCommandBuffer;

pub mod future;
use future::VglFuture;


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

        let future = VglFuture::new(
            &logical_device,
        );

        Self {
            event_loop,
            surface,

            logical_device,

            swapchain,

            triangle: None,
            setup: None,

            render_pass,

            pipeline,

            viewport,
            framebuffers,

            future,

            recreate_swapchain: false,
        }
    }


    pub fn run(mut self) {
        self.setup.unwrap()(&mut self);

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
                    self.future.cleanup();

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


                    let swapchain_image = VglSwapchainImage::new(&self.swapchain);
                    if swapchain_image.suboptimal() { return; }

                    let command_buffer = VglCommandBuffer::new(
                        &self.logical_device,
                        &self.pipeline,
                        &self.viewport,
                        &self.framebuffers,
                        &swapchain_image,
                        self.triangle.as_ref().unwrap(),
                    );

                    self.future.update_future(
                        &self.logical_device,
                        &self.swapchain,
                        swapchain_image,
                        command_buffer,
                    );
                }
                _ => (),
            }
        });
    }
}
