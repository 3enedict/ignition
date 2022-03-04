extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn render(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        use wgpu::RenderPass;

        let mut commands = match Commands::ignite(&engine) {
            Ok(commands) => commands,
            Err(wgpu::SurfaceError::Lost) => { engine.resize(engine.window.size); return; },
            Err(wgpu::SurfaceError::OutOfMemory) => { *control_flow = ControlFlow::Exit; return; },
            Err(e) => { eprintln!("{:?}", e); return; },
        };

        {
            let mut render_pass = commands.ignite_render_pass();

            #input
        }

        commands.execute(&engine);
    };

    gen.into()
}
