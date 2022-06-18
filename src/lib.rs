pub mod ecs;
pub mod options;
pub mod prelude;
pub mod renderer;

use crate::ecs::Scene;
use crate::options::Options;
use crate::renderer::Renderer;

pub struct Engine {
    pub options: Options,

    pub renderer: Renderer,
    pub scene: Scene,
}

impl Engine {
    pub fn ignite() -> Self {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        pollster::block_on(Engine::setup_engine())
    }

    pub async fn setup_engine() -> Engine {
        Self {
            options: Options::default(),

            renderer: Renderer::new(),
            scene: Scene::new(),
        }
    }
}
