use vulkano::device::DeviceExtensions;
use vulkano::pipeline::graphics::viewport::Viewport;
use winit::event_loop::EventLoop;

use crate::core::VglRenderer;

pub mod validation_layers;

pub mod instance;
use crate::core::rendering::instance::create_instance;

pub mod surface;
use crate::core::rendering::surface::create_surface;

pub mod physical_device;
use crate::core::rendering::physical_device::create_physical_device;

pub mod logical_device;
use crate::core::rendering::logical_device::create_logical_device;

pub mod swapchain;
use crate::core::rendering::swapchain::create_swapchain;
use crate::core::rendering::swapchain::recreate_swapchain;

pub mod render_pass;
use crate::core::rendering::render_pass::create_render_pass;

pub mod pipeline;
use crate::core::rendering::pipeline::create_graphics_pipeline;

pub mod basic_shaders;

pub mod framebuffers;
use crate::core::rendering::framebuffers::create_framebuffers;

pub mod swapchain_image;
use crate::core::rendering::swapchain_image::create_swapchain_images;

pub mod command_buffer;
use crate::core::rendering::command_buffer::create_command_buffer;

pub mod future;
use crate::core::rendering::future::create_future;
use crate::core::rendering::future::update_future;
use crate::core::rendering::future::cleanup_future;

pub fn create_renderer() -> VglRenderer {
    let instance = create_instance();




    let event_loop = EventLoop::new();

    let surface = create_surface(
        &instance,
        &event_loop,
    );




    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    let (physical_device, queue_family) = create_physical_device(
        &instance,
        &surface,
        &device_extensions,
    );

    let (logical_device, queue) = create_logical_device(
        &device_extensions,
        &physical_device,
        &queue_family,
    );




    let (swapchain, swapchain_images) = create_swapchain(
        &surface,
        &physical_device,
        &logical_device,
        &queue,
    );




    let vs = basic_shaders::vs::load(logical_device.clone()).unwrap();
    let fs = basic_shaders::fs::load(logical_device.clone()).unwrap();

    let render_pass = create_render_pass(
        &logical_device,
        &swapchain,
    );

    let pipeline = create_graphics_pipeline(
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

    let framebuffers = create_framebuffers(
        &swapchain,
        &swapchain_images,
        &render_pass,
        &mut viewport,
    );

    let future = create_future(
        &logical_device,
    );

    VglRenderer {
        event_loop: Some(event_loop),
        surface,

        logical_device,
        queue,

        swapchain,

        objects: Vec::new(),

        render_pass,

        pipelines: vec![pipeline],

        viewport,
        framebuffers,

        future,

        recreate_swapchain: false,
    }
}

pub fn draw(
    renderer: &mut VglRenderer,
) {
    cleanup_future(&mut renderer.future);

    if renderer.recreate_swapchain {
        let (recreate_swapchain, swapchain_and_images) = recreate_swapchain(&renderer.surface, &renderer.swapchain);
        if recreate_swapchain { return; }

        let (swapchain, swapchain_images) = swapchain_and_images.unwrap();

        renderer.framebuffers = create_framebuffers(
            &swapchain,
            &swapchain_images,
            &renderer.render_pass,
            &mut renderer.viewport,
        );

        renderer.swapchain = swapchain;
        renderer.recreate_swapchain = false;
    }


    let (swapchain_image_index, future) = create_swapchain_images(&renderer.swapchain);
    if future.is_none() { return; }

    let command_buffer = create_command_buffer(
        &renderer.logical_device,
        &renderer.queue,
        &renderer.pipelines,
        &renderer.viewport,
        &renderer.framebuffers,
        swapchain_image_index,
        &mut renderer.objects,
    );

    update_future(
        renderer,
        future,
        swapchain_image_index,
        command_buffer,
    );
}
