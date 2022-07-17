pub struct VertexGroup {}

#[cfg(test)]
mod tests {
    use bytemuck::bytes_of;

    use crate::manifestation::apex::VertexGroup;

    #[test]
    fn vertex_data_gets_seperated_and_casted_correctly() {
        let vertex_group = VertexGroup::new();
        vertex_group.data([0.55, -0.5, 0.55, 0.55, -0.5, 0.55], 2);

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
