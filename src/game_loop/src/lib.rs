extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn game_loop(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        use winit::event_loop::ControlFlow;
        use winit::event::{Event, WindowEvent};

        renderer.event_loop.take().unwrap().run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    renderer.recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    #input
                }
                _ => (),
            }
        });
    };

    gen.into()
}
