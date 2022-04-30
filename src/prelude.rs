pub use wgpu::include_wgsl;

pub use crate::{
    core::{
        rendering::{command_buffer::Commands, vertex_buffer::Vertex},
        shapes::{crackers::crackers, doritos::doritos, indexed_shape, shape},
    },
    ecs::IgnitionScene,
    Engine,
};

pub use game_loop::game_loop;

pub use any_thread::any_thread;
pub use event_driven::event_driven;

pub use draw::draw;
pub use render::render;

pub use run::run;
pub use run_return::run_return;

pub use handle_default_events::handle_default_events;
pub use main_events_cleared::main_events_cleared;
pub use redraw::redraw;
pub use redraw_requested::redraw_requested;
