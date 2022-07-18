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
}
