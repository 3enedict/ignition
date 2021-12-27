use vulkano::device::DeviceExtensions;
use vulkano::pipeline::graphics::viewport::Viewport;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};

use crate::renderer::VglRenderer;
use crate::objects::VglObjects;

pub mod parameters;
use parameters::VglRendererParameters;

pub mod validation_layers;
use validation_layers::VglValidationLayers;

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
    layout(location = 0) in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
      "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
    #version 450
    layout(location = 0) out vec4 outColor;

    void main() {
        outColor = vec4(0.6, 0.6, 0.6, 1.0);
    }
            "
    }
}

/*
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
    #version 450
    #extension GL_ARB_separate_shader_objects : enable

    layout(location = 0) out vec3 fragColor;

    layout(location = 0) in vec2 position;

    vec3 colors[3] = vec3[](
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 1.0)
    );

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        fragColor = colors[gl_VertexIndex % 3];
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
*/

impl VglRenderer {
    pub fn new(
        parameters: VglRendererParameters,
    ) -> Self {
        let mut validation_layers = VglValidationLayers::new();

        let instance = VglInstance::new(&validation_layers);

        validation_layers.setup_debug_callback(&instance);

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

        let vs = vs::load(logical_device.clone_logical_device()).unwrap();
        let fs = fs::load(logical_device.clone_logical_device()).unwrap();

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

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        let framebuffers = VglFramebuffers::new(
            &swapchain,
            &render_pass,
            &mut viewport,
        );

        let future = VglFuture::new(
            &logical_device,
        );

        Self {
            parameters,

            event_loop: Some(event_loop),
            surface,

            logical_device,

            swapchain,

            objects: VglObjects::new(),

            render_pass,

            pipeline,

            viewport,
            framebuffers,

            future,

            recreate_swapchain: false,
        }
    }

    pub fn draw(
        &mut self,
    ) {
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
            &mut self.objects,
        );

        self.future.update_future(
            &self.logical_device,
            &self.swapchain,
            swapchain_image,
            command_buffer,
        );
    }


    pub fn run(mut self) {
        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
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
                    self.draw();
                }
                _ => (),
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::renderer::VglRenderer;
    use crate::renderer::core::parameters::VglRendererParameters;

    use crate::objects::vertex::Vertex;

    fn one_triangle(renderer: &mut VglRenderer) {
        let mut triangle = vec!
            [
            Vertex { position: [ 0.0, -0.5] },
            Vertex { position: [ 0.5,  0.5] },
            Vertex { position: [-0.5,  0.5] },
            ];

        renderer.add_triangles(&mut triangle);
    }

    fn two_triangles(renderer: &mut VglRenderer) {
        let mut triangles = vec!
            [
            Vertex { position: [ 0.55, -0.5 ] },
            Vertex { position: [ 0.55,  0.55] },
            Vertex { position: [-0.5 ,  0.55] },

            Vertex { position: [-0.55,  0.5 ] },
            Vertex { position: [-0.55, -0.55] },
            Vertex { position: [ 0.5 , -0.55] },
            ];

        renderer.add_triangles(&mut triangles);
    }

    #[test]
    fn render_one_triangle() {
        VglRenderer::new(VglRendererParameters::default())
            .add_system_setup(one_triangle)
            .draw();
    }

    #[test]
    fn render_two_triangles() {
        VglRenderer::new(VglRendererParameters::default())
            .add_system_setup(two_triangles)
            .draw();
    }
}
