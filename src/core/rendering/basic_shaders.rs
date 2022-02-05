pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 450
            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 450
            layout(location = 0) out vec4 outColor;

            void main() {
                outColor = vec4(0.6, 0.6, 0.6, 1.0);
            }
        "
    }
}