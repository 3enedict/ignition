use wgpu::{
    Buffer,
    BufferUsages,

    VertexBufferLayout,
    VertexAttribute,
    VertexFormat,
    VertexStepMode,

    util::{BufferInitDescriptor, DeviceExt},
};

use crate::core::Engine;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ]
        }
    }
}



pub fn ignite_vertex_buffer(engine: &mut Engine, vertices: &Vec<Vertex>) -> Buffer {
    let vertex_buffer = engine.gpu.device.create_buffer_init(
        &BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        }
    );

    vertex_buffer
}