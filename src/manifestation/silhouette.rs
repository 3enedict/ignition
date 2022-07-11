use wgpu::RenderPass;

pub mod crackers;
pub mod doritos;

pub trait Renderable {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>);
}
