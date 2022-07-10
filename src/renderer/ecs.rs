use wgpu::ShaderModuleDescriptor;

use crate::{
    renderer::core::vertex_buffer::{XYRGB, XYZRGB},
    Engine,
};

#[derive(Debug)]
pub struct XY {
    pos: [f32; 2],
}

#[derive(Debug)]
pub struct XYZ {
    pos: [f32; 3],
}

#[derive(Debug)]
pub struct Color {
    color: [f32; 3],
}

impl Engine {
    pub fn with_component<G: 'static>(&mut self, component: G) -> &mut Self {
        let entity = self.scene.get_current_entity();
        self.scene.component(entity, component);

        self
    }

    pub fn xy<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();

        for pos in coordinates.windows(2).step_by(2) {
            let [x, y]: [f32; 2] = pos.try_into().unwrap();
            let xy = XY { pos: [x, y] };

            self.scene.vectorized_component(entity, xy);
        }

        self
    }

    pub fn xyz<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();
        println!("Hello...");

        for pos in coordinates.windows(3).step_by(3) {
            let [x, y, z]: [f32; 3] = pos.try_into().unwrap();
            let xyz = XYZ { pos: [x, y, z] };

            self.scene.vectorized_component(entity, xyz);
        }

        self
    }

    pub fn rgb<const N: usize>(&mut self, colors: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();

        for color in colors.windows(3).step_by(3) {
            let [r, g, b]: [f32; 3] = color.try_into().unwrap();
            let rgb = Color { color: [r, g, b] };

            self.scene.vectorized_component(entity, rgb);
        }

        self
    }

    pub fn entity(&mut self) -> usize {
        self.scene.entity()
    }

    pub fn doritos(&mut self) -> usize {
        let entity = self.scene.get_current_entity();

        if self.scene.component_exists::<Vec<XY>>(entity) {
            let mut vertices = Vec::new();
            let positions = self.scene.get_component::<Vec<XY>>(entity);
            let colors = self.scene.get_component::<Vec<Color>>(entity);

            for i in 0..positions.len() {
                vertices.push(XYRGB {
                    position: positions[i].pos,
                    color: colors[i].color,
                });
            }

            let doritos = self.renderer.doritos(
                &vertices,
                self.scene.get_component::<ShaderModuleDescriptor>(entity),
            );

            self.scene.component(entity, doritos);
            return self.scene.entity();
        } else if self.scene.component_exists::<Vec<XYZ>>(entity) {
            let mut vertices = Vec::new();
            let positions = self.scene.get_component::<Vec<XYZ>>(entity);
            let colors = self.scene.get_component::<Vec<Color>>(entity);

            for i in 0..positions.len() {
                vertices.push(XYZRGB {
                    position: positions[i].pos,
                    color: colors[i].color,
                });
            }

            let doritos = self.renderer.doritos(
                &vertices,
                self.scene.get_component::<ShaderModuleDescriptor>(entity),
            );

            self.scene.component(entity, doritos);
            return self.scene.entity();
        } else {
            unimplemented!()
        }
    }
}
