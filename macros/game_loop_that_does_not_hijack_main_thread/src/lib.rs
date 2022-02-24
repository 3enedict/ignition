extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn game_loop_that_does_not_hijack_main_thread(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        extern crate winit;
        extern crate wgpu;

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

                    #input

                    match event {
                        Event::WindowEvent {
                            event: WindowEvent::Resized(size),
                            ..
                        } => {
                            engine.resize(size);
                        }

                        Event::RedrawRequested(_) => {
                            match engine.render() {
                                Ok(_) => {}
                                Err(SurfaceError::Lost) => engine.configure_surface(),
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

            if engine.window.event_loop.is_none() {
                return;
            }
        }
    };

    gen.into()
}
