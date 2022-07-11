use wgpu::ShaderModuleDescriptor;

use crate::{
    manifestation::apex::{xyrgb::XYRGB, xyzrgb::XYZRGB, RGB, XY, XYZ},
    Engine,
};

impl Engine {
    pub fn component<G: 'static>(&mut self, component: G) -> &mut Self {
        let entity = self.scene.get_current_entity();
        self.scene.component(entity, component);

        self
    }

    pub fn xy<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();

        for pos in coordinates.windows(2).step_by(2) {
            let xy = XY {
                xy: pos.try_into().unwrap(),
            };

            self.scene.vectorized_component(entity, xy);
        }

        self
    }

    pub fn xyz<const N: usize>(&mut self, coordinates: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();

        for pos in coordinates.windows(3).step_by(3) {
            let xyz = XYZ {
                xyz: pos.try_into().unwrap(),
            };

            self.scene.vectorized_component(entity, xyz);
        }

        self
    }

    pub fn rgb<const N: usize>(&mut self, colors: [f32; N]) -> &mut Self {
        let entity = self.scene.get_current_entity();

        for color in colors.windows(3).step_by(3) {
            let rgb = RGB {
                rgb: color.try_into().unwrap(),
            };

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
                self.scene.get_component::<ShaderModuleDescriptor>(entity),
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
                self.scene.get_component::<ShaderModuleDescriptor>(entity),
            );

            self.scene.component(entity, doritos);
            return self.scene.entity();
        } else {
            unimplemented!()
        }
    }
}
