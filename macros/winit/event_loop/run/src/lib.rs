extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn run(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        engine.window.event_loop
            .take().unwrap()
            .run(move |event, _, control_flow| {
                let _ = &engine;
                *control_flow = engine.options.control_flow;

                handle_default_events!();

                #input
            });
    };

    gen.into()
}
