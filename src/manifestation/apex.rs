use wgpu::VertexBufferLayout;

pub mod xyrgb;
pub mod xyzrgb;

pub trait Vertex {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}

pub trait VertexData {
    type Data;

    fn new(data: &[Self::Data]) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct XY {
    pub xy: [f32; 2],
}

impl VertexData for XY {
    type Data = f32;

    fn new(data: &[Self::Data]) -> Self {
        Self {
            xy: data.try_into().unwrap(),
        }
    }
}

pub struct XYZ {
    pub xyz: [f32; 3],
}

pub struct RGB {
    pub rgb: [f32; 3],
}
