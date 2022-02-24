extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn run_return(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        use wgpu::SurfaceError;

        use winit::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
            platform::run_return::EventLoopExtRunReturn
        };

        println!("WARNING: This macro should not be used as it is platform dependent !");

        loop {
            engine.window.event_loop
                .take().unwrap()
                .run_return(|event, _, control_flow| {
                    let _ = &engine;
                    *control_flow = engine.options.control_flow;

                    handle_events!();

                    #input
                });

            if engine.window.event_loop.is_none() {
                return;
            }
        }
    };

    gen.into()
}
