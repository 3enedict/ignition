extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn draw(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input).into_iter();

    let gen = quote! {
        #(#input.render(&mut render_pass);)* 
    };

    gen.into()
}
