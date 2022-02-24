extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn game_loop(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        use wgpu::SurfaceError;

        use winit::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
            platform::run_return::EventLoopExtRunReturn
        };

        engine.window.event_loop
            .take().unwrap()
            .run(move |event, _, control_flow| {
                let _ = &engine;
                *control_flow = engine.options.control_flow;

                handle_events!();

                #input
            });
    };

    gen.into()
}
