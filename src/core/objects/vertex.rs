#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
  pub position: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position);
