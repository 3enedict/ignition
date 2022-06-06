pub use wgpu::include_wgsl;

pub use crate::{
    core::{
        rendering::{command_buffer::Commands, vertex_buffer::Vertex},
        shapes::{crackers::crackers, doritos::doritos, indexed_shape, shape},
    },
    ecs::IgnitionScene,
    Engine,
};
