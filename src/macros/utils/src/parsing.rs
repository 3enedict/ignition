use heck::AsSnakeCase;
use lazy_static::lazy_static;
use proc_macro2::{Ident, Span};
use regex::Regex;

use crate::accessors::{get_components, get_current_crate};

pub fn parse_components() -> Vec<(String, String)> {
    let components = get_components();

    lazy_static! {
        static ref RE: Regex = Regex::new("(.*) = \"(.*)\"").unwrap();
    }

    RE.captures_iter(&components)
        .map(|x| (x[1].to_string(), x[2].to_string()))
        .collect::<Vec<(String, String)>>()
}

pub fn get_path_of_engine_in_current_crate() -> String {
    let regex = Regex::new(&format!(r"(?m)engine = {}()$", get_current_crate())).unwrap();
    if let Some(cap) = regex.captures(&get_components()) {
        return format!("{}{}", get_current_crate(), cap[1].to_string());
    }

    String::new()
}

pub fn to_ident(string: &String) -> Ident {
    Ident::new(string.as_str(), Span::call_site())
}

pub fn to_snakecased_ident(string: &String) -> Ident {
    Ident::new(
        format!("{}", AsSnakeCase(string)).as_str(),
        Span::call_site(),
    )
}
