pub use wgpu::include_wgsl;

pub use crate::core::{
    Engine,
    rendering::vertex_buffer::Vertex,
    shapes::ignite_shape,
};

pub use run_return::run_return;
pub use redraw_requested::redraw_requested;
pub use handle_events::handle_events;
pub use render::render;
