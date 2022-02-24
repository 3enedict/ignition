extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn render(_raw_input: TokenStream) -> TokenStream {
    let gen = quote! {
        match engine.render() {
            Ok(_) => {}
            Err(SurfaceError::Lost) => engine.configure_surface(),
            Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        }
    };

    gen.into()
}
