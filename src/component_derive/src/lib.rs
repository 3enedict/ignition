use std::{env, fs, io::prelude::*, path::PathBuf};

use proc_macro::TokenStream;
use syn;

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let name = get_struct_name(&input);
    let data = get_list_of_components_from_file();

    if data.find(&name) != None {
        write_component_name_to_file(name);
    }

    input
}

//-----------------------------------------------------------

fn tempfile() -> PathBuf {
    let mut tempfile = env::temp_dir();
    tempfile.push("components.toml");

    tempfile
}

fn get_struct_name(input: &TokenStream) -> String {
    let ast: syn::ItemStruct = syn::parse(input.clone()).unwrap();
    let name = ast.ident;

    name.to_string()
}

fn get_list_of_components_from_file() -> String {
    match fs::read_to_string("components.toml") {
        Ok(ids) => ids,
        Err(_) => match fs::read_to_string(tempfile().as_path()) {
            Ok(ids) => ids,
            Err(_) => String::new(),
        },
    }
}

fn components_file() -> fs::File {
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("components.toml")
        .unwrap()
}

fn write_component_name_to_file(name: String) {
    if let Err(e) = writeln!(components_file(), "{}", name) {
        eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(_) = fs::copy("components.toml", tempfile()) {
        println!("Unable to copy list of components to temporary file");
    }
}
