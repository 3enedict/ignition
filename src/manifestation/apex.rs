pub struct VertexGroup<'a> {
    data: Vec<Vec<&'a [u8]>>,
}

impl<'a> VertexGroup<'a> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn data<G: bytemuck::Pod>(&mut self, data: &'a [G], step: usize) {
        for (i, point) in data.windows(step).step_by(step).enumerate() {
            if self.data.get(i).is_none() {
                self.data.push(Vec::with_capacity(step))
            }

            for value in point.into_iter() {
                self.data[i].push(bytemuck::bytes_of(value));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytemuck::bytes_of;
    use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

    use crate::manifestation::apex::VertexGroup;

    #[test]
    fn vertex_data_gets_seperated_and_casted_correctly() {
        let mut vertex_group = VertexGroup::new();
        vertex_group.data(&[0.55, -0.5, 0.55, 0.55, -0.5, 0.55], 2);

        assert_eq!(
            vertex_group.data,
            vec![
                vec![bytes_of(&0.55), bytes_of(&-0.5)],
                vec![bytes_of(&0.55), bytes_of(&0.55)],
                vec![bytes_of(&-0.5), bytes_of(&0.55)]
            ]
        );
    }

    #[test]
    fn different_types_of_vertex_data_get_processed_correctly() {
        let mut vertex_group = VertexGroup::new();
        vertex_group.data(&[0.55, -0.5, 0.55, 0.55, -0.5, 0.55], 2);
        vertex_group.data(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3);

        assert_eq!(
            vertex_group.data,
            vec![
                vec![
                    bytes_of(&0.55),
                    bytes_of(&-0.5),
                    bytes_of(&1.0),
                    bytes_of(&0.0),
                    bytes_of(&0.0)
                ],
                vec![
                    bytes_of(&0.55),
                    bytes_of(&0.55),
                    bytes_of(&0.0),
                    bytes_of(&1.0),
                    bytes_of(&0.0)
                ],
                vec![
                    bytes_of(&-0.5),
                    bytes_of(&0.55),
                    bytes_of(&0.0),
                    bytes_of(&0.0),
                    bytes_of(&1.0)
                ],
            ]
        );
    }

    #[test]
    fn layout_is_generated_correctly() {
        let mut vertex_group = VertexGroup::new();
        vertex_group.data(&[0.55, -0.5, 0.55, 0.55, -0.5, 0.55], 2);
        vertex_group.data(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3);

        assert_eq!(
            vertex_group.layout(),
            VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 5]> as BufferAddress,
                step_mode: VertexStepMode::Vertex,
                attributes: &[
                    VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: VertexFormat::Float32x2,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 2]> as BufferAddress,
                        shader_location: 1,
                        format: VertexFormat::Float32x3,
                    }
                ]
            }
        );
    }
}
