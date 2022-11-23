use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

use regex::Regex;
use syn::ItemUse;

use utils::*;

#[proc_macro]
pub fn engine(_input: TokenStream) -> TokenStream {
    let components = update_components().unwrap_or(parse_components());

    let types = convert_type_names(&components, |x| to_ident(x));
        let types_trait = convert_type_names(&components, |x| to_ident(&format!("{}Trait", x)));
    let names = convert_type_names(&components, |x| to_snakecased_ident(&x));
    let names_mut = convert_type_names(&components, |x| to_snakecased_ident(&format!("{}_mut", x)));

    let regex = Regex::new(&format!(r"{}::\{{.*\}}", get_path_of_engine_in_current_crate())).unwrap();
    let paths: Vec<ItemUse> = components.iter().filter(|(_x, y)| !regex.is_match(y)).map(|(_x, y)| syn::parse_str::<ItemUse>(&format!("use {};", y.replace(&get_current_crate(), "crate"))).unwrap()).collect();


    quote! {
        #(#paths)*

        pub struct ComponentPools {
            #(pub #names : ComponentPool<#types>),*
        }

        impl ComponentPoolsTrait for ComponentPools {
            fn new() -> Self {
                Self {
                    #(#names : ComponentPool::empty()),*
                }
            }

            fn delete_entity(&mut self, entity: usize) {
                #(self.#names.delete_entity(entity);)*
            }
        }
        
        #(impl #types_trait for ComponentPools { fn #names(&self) -> &ComponentPool<#types> { &self.#names } fn #names_mut(&mut self) -> &mut ComponentPool<#types> { &mut self.#names }})*

        pub struct Engine {
            pub renderer: Screen,
            pub scene: Scene<ComponentPools>,

            pub config: RuntimeConfiguration,
        }

        impl Engine {
            pub fn ignite() -> Self {
                let renderer = Screen::new();
                let scene = Scene::new();

                let mut config = RuntimeConfiguration::default();
                config.size = renderer.window.inner_size();

                Self {
                    renderer,
                    scene,

                    config,
                }
            }
        }
    }
    .into()
}


fn convert_type_names(components: &Vec<(String, String)>, closure: impl Fn(&String) -> Ident) -> Vec<Ident> {
    components
        .iter()
        .map(|(x, _y)| closure(x))
        .collect()
}
