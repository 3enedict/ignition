use std::{env, fs, io::prelude::*, path::PathBuf};

use heck::AsSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemStruct = syn::parse(input.clone()).unwrap();

    let ident = ast.ident.clone();
    let name = Ident::new(
        format!("{}", AsSnakeCase(ident.clone().to_string())).as_str(),
        Span::call_site(),
    );
    let ident_trait = Ident::new(format!("{}Trait", ident).as_str(), Span::call_site());
    let name_mut = Ident::new(format!("{}_mut", name).as_str(), Span::call_site());
    let data = get_list_of_components_from_file();

    if data.find(&ident.to_string()) == None {
        write_component_name_to_file(&ident.to_string());
    }

    quote! {
        #ast

        pub trait #ident_trait {
            fn #name(&self) -> &ComponentPool<#ident>;
            fn #name_mut(&mut self) -> &mut ComponentPool<#ident>;
        }

        impl<G: #ident_trait> Component<G> for #ident {
            fn get_from(component_pools: &G) -> &ComponentPool<#ident> {
                component_pools.#name()
            }

            fn get_mut_from(component_pools: &mut G) -> &mut ComponentPool<#ident> {
                component_pools.#name_mut()
            }
        }
    }
    .into()
}

//-----------------------------------------------------------

fn tempfile() -> PathBuf {
    let mut tempfile = env::temp_dir();
    tempfile.push("components.toml");

    tempfile
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

fn write_component_name_to_file(name: &String) {
    if let Err(e) = writeln!(components_file(), "{}", name) {
        eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(_) = fs::copy("components.toml", tempfile()) {
        println!("Unable to copy list of components to temporary file");
    }
}
