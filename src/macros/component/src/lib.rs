use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn;

use utils::{parsing::*, update_components};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    update_components();

    let ast: syn::ItemStruct = syn::parse(input.clone()).unwrap();
    let (ident, ident_trait, name, name_mut) = get_data(&ast);

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

fn get_data(ast: &syn::ItemStruct) -> (Ident, Ident, Ident, Ident) {
    let ident = ast.ident.clone();
    let ident_trait = to_ident(&format!("{}Trait", ident));
    let name = to_snakecased_ident(&format!("{}", ident));
    let name_mut = to_ident(&format!("{}_mut", name));

    (ident, ident_trait, name, name_mut)
}
