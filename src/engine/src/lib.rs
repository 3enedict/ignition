use std::{env, fs, io::Write, path::PathBuf, time::SystemTime};

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use heck::AsSnakeCase;
use regex::Regex;
use syn::ItemUse;

#[proc_macro]
pub fn engine(_input: TokenStream) -> TokenStream {
    let mut components: Vec<(String, String)> = Vec::new();
    let components_file = get_previous_list_of_components();

    let regex = Regex::new(r"\[\[ignition.(\d*)\]\]").unwrap();

    let mut time_of_previous_sync = 0;
    if let Some(cap) = regex.captures(&components_file) {
        time_of_previous_sync = cap[1].parse::<u64>().unwrap();
    }

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if current_time - time_of_previous_sync > 2 {
        components = get_components();

        let mut formatted_components = components
            .iter()
            .map(|(x, y)| format!("{} = \"{}\"", x, y))
            .collect::<Vec<String>>()
            .join("\n");

        formatted_components.insert_str(0, &format!("[[ignition.{}]]\n", current_time));
        formatted_components.push('\n');

        let ignition_regex = Regex::new(r"(?s)\[\[ignition.\d*\]\]\n\n.*\[*").unwrap();
        let updated_components = match ignition_regex.is_match(&components_file) {
            true => ignition_regex
                .replace(&components_file, formatted_components)
                .to_string(),
            false => formatted_components,
        };

        if let Err(_) = get_components_toml().write_all(updated_components.as_bytes()) {
            println!("Unable to write to components.toml")
        }

        if let Err(_) = fs::copy("components.toml", get_temp_components_toml()) {
            println!("Unable to copy list of components to temporary file");
        }
    }

    let current_crate = env::current_dir()
        .unwrap() // PathBuf { "/home/user/Projects/ignition/" }
        .components() // Iterator: home -> user -> Projects -> ignition
        .last()
        .unwrap() // ignition
        .as_os_str() // Endless conversions...
        .to_str()
        .unwrap()
        .to_string();

    let types: Vec<Ident> = components
        .iter()
        .map(|(x, _y)| Ident::new(x, Span::call_site()))
        .collect();

    let names: Vec<Ident> = components
        .iter()
        .map(|(x, _y)| Ident::new(format!("{}", AsSnakeCase(x)).as_str(), Span::call_site()))
        .collect();

    let mut paths: Vec<ItemUse> = Vec::new();

    for (_component, path) in components.iter() {
        if let Ok(final_path) = syn::parse_str::<ItemUse>(&path.replace(&current_crate, "crate")) {
            paths.push(final_path);
        }
    }
    
        let types_trait: Vec<Ident> = components.iter().map(|(x, _y)| Ident::new(format!("{}Trait", x).as_str(), Span::call_site())).collect();
    let names_mut: Vec<Ident> = 
components
        .iter()
        .map(|(x, _y)| Ident::new(format!("{}_mut", AsSnakeCase(x)).as_str(), Span::call_site()))
        .collect();

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

//-----------------------------------------------------------

fn get_components() -> Vec<(String, String)> {
    let mut source_directory = PathBuf::new();
    source_directory.push(".");
    source_directory.push("src");

    /* Catches names of structs definitions similar to:
            #[conponent]  // .*#\[component.*\].*\n
            #[derive(Debug)] // (.*#.*\n)*
            pub struct Int { // .*struct (.*) \{
                int: i32,
            }
    */
    let regex = Regex::new(r".*#\[component.*\].*\n(.*#.*\n)*.*struct (.*)[\{\(]").unwrap();

    let mut components = Vec::new();
    scan_directory_for_components(&source_directory, &regex, &mut components);

    components
}

fn get_temp_components_toml() -> PathBuf {
    let mut tempfile = env::temp_dir();
    tempfile.push("components.toml");

    tempfile
}

fn get_components_toml() -> fs::File {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("components.toml")
        .unwrap()
}

fn get_previous_list_of_components() -> String {
    match fs::read_to_string("components.toml") {
        Ok(ids) => ids,
        Err(_) => match fs::read_to_string(get_temp_components_toml().as_path()) {
            Ok(ids) => ids,
            Err(_) => String::new(),
        },
    }
}

fn scan_directory_for_components(
    root: &PathBuf,
    regex: &Regex,
    components: &mut Vec<(String, String)>,
) {
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            scan_directory_for_components(&path, &regex, components);
        } else {
            get_components_from_file(&path, &regex, components);
        }
    }
}

fn get_components_from_file(path: &PathBuf, regex: &Regex, components: &mut Vec<(String, String)>) {
    let src = fs::read_to_string(path).unwrap();

    for cap in regex.captures_iter(&src) {
        let name = String::from(&cap[2]).trim_matches(' ').to_string();
        let mut module_path = String::new();

        if !src.contains("engine!(") {
            let current_crate = env::current_dir()
                .unwrap() // PathBuf { "/home/user/Projects/ignition/" }
                .components() // Iterator: home -> user -> Projects -> ignition
                .last()
                .unwrap() // ignition
                .as_os_str() // Endless conversions...
                .to_str()
                .unwrap()
                .to_string();

            module_path = path // ./src/life/genesis.rs
                .components() // Iterator: . -> src -> life -> genesis.rs
                .map(|x| x.as_os_str().to_str().unwrap()) // Conversion to &str
                .collect::<Vec<&str>>()
                .join("::") // ".::src::life::genesis.rs"
                .replace(".::src", &format!("use {}", &current_crate)) // "use ignition::life::genesis.rs"
                .replace(".rs", &format!("::{{{}, {}Trait}};", name, name)) // "use ignition::life::genesis::{Name, NameTrait};"
                .replace("lib::", ""); // if "use ignition::lib::Name;" then "use ignition::Name;"
        }
        
        components.push((name, module_path));
    }
}
