extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn render(raw_input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(raw_input);

    let gen = quote! {
        use ignition::core::rendering::command_buffer::{create_frame, create_command_encoder, create_render_pass};

        let (frame, view) = create_frame(&engine);

        let mut encoder = create_command_encoder(&engine);

        {
            let mut render_pass = create_render_pass(&mut encoder, &view);

            #input
        }

        let command_buffer = Some(encoder.finish());
        engine.gpu.queue.submit(command_buffer);

        frame.present();

    };

    gen.into()
}
