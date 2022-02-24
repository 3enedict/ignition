extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn handle_events(_raw_input: TokenStream) -> TokenStream {
    let gen = quote! {
        use winit::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
        };

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                engine.resize(size);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    };

    gen.into()
}
