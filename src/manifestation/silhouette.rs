use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, Face, FragmentState,
    FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    VertexState,
};

use crate::Engine;

impl Engine {
    pub fn vertex_buffer(&mut self, vertices: Vec<f32>) -> Buffer {
        // Note: Probably replacable with .map()
        let mut contents: Vec<u8> = Vec::new();
        for value in vertices.into_iter() {
            contents.append(&mut bincode::serialize(&value).unwrap());
        }

        let vertex_buffer = self
            .renderer
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &contents,
                usage: BufferUsages::VERTEX,
            });

        vertex_buffer
    }

    pub fn pipeline<'a>(&mut self, shaders: ShaderModuleDescriptor) -> RenderPipeline {
        let shader = self.renderer.device.create_shader_module(shaders);

        let pipeline_layout =
            self.renderer
                .device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let pipeline = self
            .renderer
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                    }],
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: self.renderer.config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        pipeline
    }
}
