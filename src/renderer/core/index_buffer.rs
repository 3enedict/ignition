use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

use crate::Engine;

pub fn ignite_index_buffer(engine: &mut Engine, indices: &Vec<u16>) -> Buffer {
    let index_buffer = engine
        .renderer
        .gpu
        .device
        .create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

    index_buffer
}
