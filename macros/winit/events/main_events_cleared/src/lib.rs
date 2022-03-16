extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn main_events_cleared(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        match event {
            Event::MainEventsCleared => {
                #input
            }
            _ => {}
        }
    };

    gen.into()
}
