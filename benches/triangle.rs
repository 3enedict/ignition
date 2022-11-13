use criterion::{criterion_group, criterion_main, Criterion};

use cgmath::*;
use ignition::{life::Scene, ComponentPools, Position, Rotation, Transform, Velocity};
use legion::World as LegionWorld;
use legion::*;
use specs::prelude::*;
use specs::World as SpecsWorld;
use specs_derive::*;

#[derive(Copy, Clone)]
struct LTransform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct LPosition(Vector3<f32>);

#[derive(Copy, Clone)]
struct LRotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct LVelocity(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct STransform(Matrix4<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct SPosition(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct SRotation(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct SVelocity(Vector3<f32>);

pub fn legion_simple_insert() {
    let mut world = LegionWorld::default();

    world.extend(
        (
            vec![LTransform(Matrix4::from_scale(1.0)); 10000],
            vec![LPosition(Vector3::unit_x()); 10000],
            vec![LRotation(Vector3::unit_x()); 10000],
            vec![LVelocity(Vector3::unit_x()); 10000],
        )
            .into_soa(),
    );
}

pub fn specs_simple_insert() {
    let mut world = SpecsWorld::new();
    world.register::<STransform>();
    world.register::<SPosition>();
    world.register::<SRotation>();
    world.register::<SVelocity>();
    (0..10000).for_each(|_| {
        world
            .create_entity()
            .with(STransform(Matrix4::<f32>::from_scale(1.0)))
            .with(SPosition(Vector3::unit_x()))
            .with(SRotation(Vector3::unit_x()))
            .with(SVelocity(Vector3::unit_x()))
            .build();
    });
}

pub fn ignition_simple_insert() {
    let mut scene: Scene<ComponentPools> = Scene::new();

    for i in 0..10000 {
        let entity = scene.entity();
        scene.component(entity, Transform(Matrix4::<f32>::from_scale(1.0)));
        scene.component(entity, Position(Vector3::unit_x()));
        scene.component(entity, Rotation(Vector3::unit_x()));
        scene.component(entity, Velocity(Vector3::unit_x()));
    }
}

fn bench_simple_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_insert");
    group.bench_function("legion", |b| {
        b.iter(move || legion_simple_insert());
    });
    group.bench_function("specs", |b| {
        b.iter(move || specs_simple_insert());
    });
    group.bench_function("ignition", |b| {
        b.iter(move || ignition_simple_insert());
    });
}

criterion_group!(benches, bench_simple_insert);
criterion_main!(benches);

/*
use ignition::prelude::*;
use wgpu::{
    util::DeviceExt, Buffer, Device, Queue, RenderPipeline, Texture, TextureDescriptor, TextureView,
};

pub fn ignition_command_buffer(
    engine: &mut Engine<Image<'static>>,
    pipeline: &mut RenderPipeline,
    vertex_buffer: &mut Buffer,
) {
    let mut commands = engine.encoder().unwrap();
    let mut render_pass = engine.render_pass(&mut commands);

    render_pass.set_pipeline(&pipeline);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_pass.draw(0..3, 0..1);

    drop(render_pass);
    engine.render(commands).unwrap();
    engine.renderer.buffer.unmap();
}

pub fn wgpu_command_buffer(
    vertex_buffer: &mut Buffer,
    device: &mut Device,
    texture_view: &mut TextureView,
    render_pipeline: &mut RenderPipeline,
    texture: &mut Texture,
    texture_desc: &mut TextureDescriptor,
    texture_size: u32,
    u32_size: u32,
    output_buffer: &mut Buffer,
    queue: &mut Queue,
) {
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        };
        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(u32_size * texture_size),
                rows_per_image: NonZeroU32::new(texture_size),
            },
        },
        texture_desc.size,
    );

    queue.submit(Some(encoder.finish()));

    // We need to scope the mapping variables so that we can
    // unmap the buffer
    {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        pollster::block_on(rx.receive()).unwrap().unwrap();

        buffer_slice.get_mapped_range();
    }

    output_buffer.unmap();
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub fn record_command_buffers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Command buffers");

    // Ignition

    let mut engine = Configuration::default().image();

    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let mut vertex_buffer = engine.vertex_buffer(vertices);
    let mut pipeline = engine.pipeline(
        include_wgsl!("shaders/gradient.wgsl"),
        engine.renderer.description.format,
    );

    group.bench_function("Ignition", |b| {
        b.iter(|| {
            ignition_command_buffer(
                black_box(&mut engine),
                black_box(&mut pipeline),
                black_box(&mut vertex_buffer),
            )
        })
    });

    // Wgpu

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();
    let (mut device, mut queue) =
        pollster::block_on(adapter.request_device(&Default::default(), None)).unwrap();

    let texture_size = 256u32;
    let mut texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: texture_size,
            height: texture_size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: None,
    };
    let mut texture = device.create_texture(&texture_desc);
    let mut texture_view = texture.create_view(&Default::default());

    // we need to store this for later
    let u32_size = std::mem::size_of::<u32>() as u32;

    let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST
            // this tells wpgu that we want to read this buffer from the cpu
            | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    let mut output_buffer = device.create_buffer(&output_buffer_desc);

    let mut wgpu_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let shader = device.create_shader_module(include_wgsl!("shaders/gradient.wgsl"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let vertex_description = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };

    let mut render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex",
            buffers: &[vertex_description],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment",
            targets: &[Some(wgpu::ColorTargetState {
                format: texture_desc.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    });

    group.bench_function("Wgpu", |b| {
        b.iter(|| {
            wgpu_command_buffer(
                black_box(&mut wgpu_vertex_buffer),
                black_box(&mut device),
                black_box(&mut texture_view),
                black_box(&mut render_pipeline),
                black_box(&mut texture),
                black_box(&mut texture_desc),
                black_box(texture_size),
                black_box(u32_size),
                black_box(&mut output_buffer),
                black_box(&mut queue),
            )
        })
    });

    group.finish();
}

pub fn triangle_creation(c: &mut Criterion) {
    let mut engine = Configuration::default().headless();

    c.bench_function("Create simple triangle", |b| {
        b.iter(|| {
            let vertices: Vec<f32> = vec![
                0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0,
                0.0, 1.0,
            ];

            engine.vertex_buffer(black_box(vertices));
            engine.pipeline(
                black_box(wgpu::include_wgsl!("shaders/gradient.wgsl")),
                black_box(wgpu::TextureFormat::Rgba8UnormSrgb),
            );
        })
    });
}
*/
