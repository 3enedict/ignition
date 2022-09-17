use std::{collections::HashMap, env, fs};

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

    let mut ids: HashMap<&str, usize> = HashMap::new();
    data.lines().for_each(|x| {
        let id: Vec<&str> = x.split(" ").collect();
        ids.insert(id[0], id[1].parse::<usize>().unwrap());
    });

    let gen = quote! {
        impl Component for #name {
            fn id() -> usize {
                return 0;
            }
        }
    };

    gen.into()
}
