use std::str::FromStr;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn parse_run_return_attribute(input_string: &mut String) -> proc_macro2::TokenStream {
    let mut run = proc_macro2::TokenStream::from_str("run!").unwrap();
    let token = "run_return";

    if input_string.contains(token) {
        *input_string = input_string.replace(&token, "");
        run = proc_macro2::TokenStream::from_str("run_return!").unwrap();
    }

    run
}

#[proc_macro]
pub fn game_loop(raw_input: TokenStream) -> TokenStream {
    let mut input_string = raw_input.to_string();

    let run = parse_run_return_attribute(&mut input_string);

    let input = proc_macro2::TokenStream::from_str(&input_string).unwrap();

    let gen = quote! {
        #run (
            redraw_requested! (
                render! (
                    #input
                );
            );

            main_events_cleared! (
                redraw!();
            );
        );
    };



    gen.into()
}
