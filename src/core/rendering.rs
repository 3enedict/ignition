use wgpu::{
    Instance,
    Backends,

    SurfaceError,
    TextureViewDescriptor,
};


use crate::core::Engine;

pub mod window;
use window::IgnitionWindow;

pub mod gpu;
use gpu::IgnitionGPU;

pub mod commands;
use commands::create_command_buffer;

impl Engine {
    pub async fn setup_engine() -> Self {
        env_logger::init();

        let instance = Instance::new(Backends::all());

        let mut window = IgnitionWindow::new(&instance);
        let gpu = pollster::block_on(IgnitionGPU::new(&instance, &window));

        window.config.format = window.surface.get_preferred_format(&gpu.adapter).unwrap();

        window.surface.configure(&gpu.device, &window.config);

        Self {
            window,
            gpu,
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let command_buffer = create_command_buffer(self, &view);

        self.gpu.queue.submit(command_buffer);
        output.present();

        Ok(())
    }
}
