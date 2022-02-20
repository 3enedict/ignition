use wgpu::{
    ShaderModule,
};

use crate::core::Engine;

struct IgnitionShaders {
    pub shaders: Vec<ShaderModule>,
}

impl IgnitionShaders {
    pub fn empty() -> Self {
        Self { shaders: Vec::new() }
    }
}
