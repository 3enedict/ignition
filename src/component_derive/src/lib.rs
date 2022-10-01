use std::{env, fs, io::prelude::*};

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Component)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component(&ast)
}

fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut tempfile = env::temp_dir();
    tempfile.push("components.toml");

    let data = match fs::read_to_string("components.toml") {
        Ok(ids) => ids,
        Err(_) => match fs::read_to_string(tempfile.as_path()) {
            Ok(ids) => ids,
            Err(_) => String::new(),
        },
    };

    let mut id = -1;
    let mut last_id = -1;
    for line in data.lines() {
        let component: Vec<&str> = line.split(" ").collect();
        let component_name = component[0];
        last_id = component[1].parse::<i32>().unwrap();

        if component_name == name.to_string() {
            id = last_id;
            break;
        }
    }

    if id == -1 {
        id = last_id + 1;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("components.toml")
            .unwrap();

        if let Err(e) = writeln!(file, "{} {}", name.to_string(), id) {
            eprintln!("Couldn't write to file: {}", e);
        }

        if let Err(_) = fs::copy("components.toml", tempfile) {
            println!("Unable to copy list of components to temporary file");
        }
    }

    let final_id = id as usize;
    let gen = quote! {
        impl Component for #name {
            fn id() -> usize {
                return #final_id;
            }
        }
    };

    gen.into()
}
