extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn game_loop(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        extern crate winit;
        extern crate wgpu;

        use wgpu::SurfaceError;

        use winit::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
        };

        renderer.window.event_loop
            .take().unwrap()
            .run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    renderer.resize(renderer.window.size);
                }

                Event::RedrawRequested(_) => {
                    #input

                    match renderer.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => renderer.resize(renderer.window.size),
                        Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        });
    };

    gen.into()
}
