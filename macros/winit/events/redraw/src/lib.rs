extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn redraw(_raw_input: TokenStream) -> TokenStream {
    let gen = quote! {
        engine.window.window.request_redraw();
    };

    gen.into()
}
