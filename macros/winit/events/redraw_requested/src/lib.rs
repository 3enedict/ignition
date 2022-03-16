extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn redraw_requested(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        match event {
            Event::RedrawRequested(_) => {
                #input
            }
            _ => {}
        }
    };

    gen.into()
}
