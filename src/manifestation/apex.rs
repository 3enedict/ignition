use wgpu::VertexBufferLayout;

pub mod xyrgb;
pub mod xyzrgb;

pub trait Vertex {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}

pub struct XY {
    pub xy: [f32; 2],
}

pub struct XYZ {
    pub xyz: [f32; 3],
}

pub struct RGB {
    pub rgb: [f32; 3],
}
