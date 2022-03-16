extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;
use regex::Regex;

#[proc_macro_attribute]
pub fn any_thread(_attr: TokenStream, raw_input: TokenStream) -> TokenStream {
    let regex = Regex::new(r"(?s:game_loop!.*?\()").unwrap();

    TokenStream::from_str(&regex.replace(&raw_input.to_string(), "game_loop! ( run_return")).unwrap()
}
