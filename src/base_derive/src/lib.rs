extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Base)]
pub fn base_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_base(&ast)
}

fn impl_base(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        use winit::event_loop::{EventLoop, ControlFlow};
        use winit::event::{Event, WindowEvent};

        use vgl::core::VglRenderer;

        impl Base for #name {
            fn run(mut self) {
                self.setup();

                self.get_renderer().take_event_loop().run(move |event, _, control_flow| {
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
                            self.get_renderer().recreate_swapchain();
                        }
                        Event::RedrawEventsCleared => {
                            self.update();
                        }
                        _ => (),
                    }
                });
            }

            fn get_renderer(&mut self) -> &mut VglRenderer {
                &mut self.view.renderer
            }
        }
    };

    gen.into()
}
