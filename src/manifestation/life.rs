use wgpu::ShaderModuleDescriptor;

use crate::{
    manifestation::apex::{xyrgb::XYRGB, xyzrgb::XYZRGB, VertexData, RGB, XY, XYZ},
    Engine,
};

impl Engine {
    pub fn component<G: 'static>(&mut self, component: G) -> &mut Self {
        let entity = self.scene.get_current_entity();
        self.scene.component(entity, component);

        self
    }

    pub fn xy<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        self.generate_vertex_data::<XY, _, N>(coordinates, 2);
        self
    }

    pub fn xyz<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        self.generate_vertex_data::<XYZ, _, N>(coordinates, 3);
        self
    }

    pub fn rgb<const N: usize>(&mut self, colors: [f32; N]) -> &mut Self {
        self.generate_vertex_data::<RGB, _, N>(colors, 3);
        self
    }

    pub fn entity(&mut self) -> usize {
        self.scene.entity()
    }

    pub fn doritos(&mut self) -> usize {
        let entity = self.scene.get_current_entity();

        /*
        if self.scene.component_exists::<Vec<XY>>(entity) {
            let mut vertices = Vec::new();
            let positions = self.scene.get_component::<Vec<XY>>(entity);
            let colors = self.scene.get_component::<Vec<RGB>>(entity);

            for i in 0..positions.len() {
                vertices.push(XYRGB {
                    position: positions[i].xy,
                    color: colors[i].rgb,
                });
            }

            println!("{:?}", vertices);

            let doritos = self.renderer.doritos(
                &vertices,
                self.scene.take_component::<ShaderModuleDescriptor>(entity),
            );

            self.scene.component(entity, doritos);
            return self.scene.entity();
        } else if self.scene.component_exists::<Vec<XYZ>>(entity) {
            let mut vertices = Vec::new();
            let positions = self.scene.get_component::<Vec<XYZ>>(entity);
            let colors = self.scene.get_component::<Vec<RGB>>(entity);

            for i in 0..positions.len() {
                vertices.push(XYZRGB {
                    position: positions[i].xyz,
                    color: colors[i].rgb,
                });
            }

            let doritos = self.renderer.doritos(
                &vertices,
                self.scene.take_component::<ShaderModuleDescriptor>(entity),
            );

            self.scene.component(entity, doritos);
        */
        return self.scene.entity();
        /*
        } else {
            unimplemented!()
        }
        */
    }

    pub fn generate_vertex_data<G: 'static + VertexData<Data = D>, D, const N: usize>(
        &mut self,
        data: [D; N],
        step: usize,
    ) {
        let entity = self.scene.get_current_entity();

        for x in data.windows(step).step_by(step) {
            self.scene.vectorized_component(entity, G::new(x));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{manifestation::apex::XY, Engine};

    #[test]
    fn vertex_data_creation_seperates_arrays_correctly() {
        let mut engine = Engine::ignite();
        engine.generate_vertex_data::<XY, _, 4>([0.34, 0.81, 0.63, 0.16], 2);

        assert_eq!(
            engine
                .scene
                .get::<Vec<XY>>()
                .iter()
                .collect::<Vec<&Vec<XY>>>(),
            vec![&vec![XY { xy: [0.34, 0.81] }, XY { xy: [0.63, 0.16] }]]
        );
    }
}
