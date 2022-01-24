extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn;

#[proc_macro_derive(Base)]
pub fn base_derive(raw_input: TokenStream) -> TokenStream {
    let input = syn::parse(raw_input).unwrap();

    impl_base(&input)
}

fn impl_base(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let renderer = find_renderer(&input.data);

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
                &mut self.#renderer
            }
        }
    };

    gen.into()
}

fn find_renderer(data: &syn::Data) -> proc_macro2::TokenStream {
    match *data {
        syn::Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    let mut renderer = proc_macro2::TokenStream::new();

                    for field in fields.named.iter() {
                        let name = &field.ident;

                        if name == &Some(syn::Ident::new("view", name.span())) {
                            renderer = quote!{view.renderer};
                        } else if name == &Some(syn::Ident::new("renderer", name.span())) {
                            renderer = quote!{renderer};
                        }
                    }

                    renderer
                }
                syn::Fields::Unit | syn::Fields::Unnamed(_) => unimplemented!(),
            }
        }
        syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
    }
}
