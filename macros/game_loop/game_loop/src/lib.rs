use std::str::FromStr;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn check_attribute(input_string: &mut String, token: &str) -> bool {
    if input_string.contains(token) {
        *input_string = input_string.replace(&token, "");
        true
    } else {
        false
    }
}

#[proc_macro]
pub fn game_loop(raw_input: TokenStream) -> TokenStream {
    let mut input_string = raw_input.to_string();

    let run = if check_attribute(&mut input_string, "run_return") { 
        proc_macro2::TokenStream::from_str("run_return!").unwrap() 
    } else { 
        proc_macro2::TokenStream::from_str("run!").unwrap() 
    };


    let event_driven = if check_attribute(&mut input_string, "event_driven") { 
        proc_macro2::TokenStream::from_str("").unwrap() 
    } else { 
        proc_macro2::TokenStream::from_str("main_events_cleared! ( redraw!(); );").unwrap() 
    };


    let input = proc_macro2::TokenStream::from_str(&input_string).unwrap();

    let gen = quote! {
        #run (
            redraw_requested! (
                render! (
                    #input
                );
            );

            #event_driven
        );
    };



    gen.into()
}
