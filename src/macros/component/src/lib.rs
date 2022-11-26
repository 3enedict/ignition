use proc_macro::TokenStream;
use quote::quote;
use syn;

use utils::{parsing::*, update_components};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    update_components();

    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let component_type = ast.ident.clone();
    let component_trait = to_ident(&format!("{}Trait", component_type));
    let component_name = to_snakecased_ident(&format!("{}", component_type));
    let component_name_mut = to_ident(&format!("{}_mut", component_name));

    quote! {
        pub trait #component_trait {
            fn #component_name(&self) -> &ComponentPool<#component_type>;
            fn #component_name_mut(&mut self) -> &mut ComponentPool<#component_type>;
        }

        impl<G: #component_trait> Component<G> for #component_type {
            fn get_from(component_pools: &G) -> &ComponentPool<#component_type> {
                component_pools.#component_name()
            }

            fn get_mut_from(component_pools: &mut G) -> &mut ComponentPool<#component_type> {
                component_pools.#component_name_mut()
            }
        }
    }
    .into()
}
