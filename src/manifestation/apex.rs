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

#[derive(Debug, PartialEq)]
pub struct XYZ {
    pub xyz: [f32; 3],
}

impl VertexData for XYZ {
    type Data = f32;

    fn new(data: &[Self::Data]) -> Self {
        Self {
            xyz: data.try_into().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RGB {
    pub rgb: [f32; 3],
}

impl VertexData for RGB {
    type Data = f32;

    fn new(data: &[Self::Data]) -> Self {
        Self {
            rgb: data.try_into().unwrap(),
        }
    }
}
