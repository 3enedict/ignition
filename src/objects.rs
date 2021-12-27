use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;


use crate::renderer::core::logical_device::VglLogicalDevice;

pub mod vertex;
use crate::objects::vertex::Vertex;

pub mod triangle;
use crate::objects::triangle::VglTriangle;
pub mod rectangle;
use crate::objects::rectangle::VglRectangle;

pub struct VglObjects {
    triangles: VglTriangle,
    rectangles: VglRectangle,
}

impl VglObjects {
    pub fn new() -> Self {
        Self {
            triangles: VglTriangle::new(),
            rectangles: VglRectangle::new(),
        }
    }

    pub fn add_triangles(
        &mut self,
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) {
        self.triangles.add(logical_device, vertices);
    }

    pub fn add_rectangles(
        &mut self,
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) {
        self.rectangles.add(logical_device, vertices, indices);
    }

    pub fn draw(
        &mut self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandPoolBuilder>,
    ) {
        self.triangles.draw(command_buffer_builder);

        self.rectangles.draw(command_buffer_builder);
    }
}
