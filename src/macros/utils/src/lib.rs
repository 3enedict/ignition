use std::{
    env,
    ffi::OsStr,
    fs::OpenOptions,
    fs::{self, File},
    io::prelude::*,
    path::Path,
    path::PathBuf,
    time::SystemTime,
};

use heck::AsSnakeCase;
use lazy_static::lazy_static;
use proc_macro2::{Ident, Span};
use regex::{Captures, Regex};

pub fn tempfile() -> PathBuf {
    let mut tempfile = env::temp_dir();
    tempfile.push("components.toml");

    tempfile
}

pub fn components_toml() -> fs::File {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("components.toml")
        .unwrap()
}

pub fn source_dir() -> PathBuf {
    let mut source_directory = PathBuf::new();
    source_directory.push(".");
    source_directory.push("src");

    source_directory
}

pub fn get_components() -> String {
    match fs::read_to_string("components.toml") {
        Ok(ids) => ids,
        Err(_) => match fs::read_to_string(tempfile().as_path()) {
            Ok(ids) => ids,
            Err(_) => String::new(),
        },
    }
}

pub fn find_components() -> Vec<(String, String)> {
    /* Catches names of structs definitions similar to:
            #[conponent]  // .*#\[component.*\].*\n
            #[derive(Debug)] // (.*#.*\n)*
            pub struct Int { // .*struct (.*) \{
                int: i32,
            }
    */
    let regex = Regex::new(r".*#\[component.*\].*\n(.*#.*\n)*.*struct (.*)[\{\(]").unwrap();

    let mut components = Vec::new();
    scan_dir_for_components(&source_dir(), &regex, &mut components);

    components
}

pub fn scan_dir_for_components(
    dir: &PathBuf,
    regex: &Regex,
    components: &mut Vec<(String, String)>,
) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            get_components_from_file(&path, &regex, components);
        } else if path.file_name() != Some(OsStr::new("macros")) {
            scan_dir_for_components(&path, &regex, components);
        }
    }
}

pub fn get_components_from_file(
    path: &PathBuf,
    regex: &Regex,
    components: &mut Vec<(String, String)>,
) {
    let src = fs::read_to_string(path).unwrap();

    if src.contains("engine!(") {
        components.push((String::from("engine"), get_module_path(&path)));
    }

    for cap in regex.captures_iter(&src) {
        let name = get_component_name(cap);
        let module_path = get_component_module_path(&path, &name);

        components.push((name, format!("\"{}\"", module_path)));
    }
}

pub fn get_component_name(capture: Captures) -> String {
    String::from(&capture[2]).trim_matches(' ').to_string()
}

pub fn get_current_crate() -> String {
    env::current_dir()
        .unwrap() // PathBuf { "/home/user/Projects/ignition/" }
        .components() // Iterator: home -> user -> Projects -> ignition
        .last() // Some("ignition")
        .unwrap() // "ignition"
        .as_os_str() // Endless conversions...
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_component_module_path(path: &PathBuf, name: &String) -> String {
    add_component_to_module_path(get_module_path(path), name)
}

pub fn get_module_path(path: &PathBuf) -> String {
    path // ./src/life/genesis.rs
        .components() // Iterator: . -> src -> life -> genesis.rs
        .map(|x| x.as_os_str().to_str().unwrap()) // Conversion to &str
        .collect::<Vec<&str>>() // vec!(".", "src", "life", "genesis.rs")
        .join("::") // ".::src::life::genesis.rs"
        .replace(".::src", &format!("{}", &get_current_crate())) // "ignition::life::genesis.rs"
        .replace(".rs", "") // "ignition::life::genesis"
        .replace("::lib", "") // if "use ignition::lib" then "use ignition"
}

pub fn add_component_to_module_path(mut module_path: String, name: &String) -> String {
    module_path.push_str(&format!("::{{{}, {}Trait}}", name, name)); // "ignition::life::genesis::{Name, NameTrait}"
    module_path
}

pub fn get_time_since_last_update() -> u64 {
    let regex = Regex::new(r"\[\[ignition.(\d*)\]\]").unwrap();

    let mut time_of_previous_sync = 0;
    if let Some(cap) = regex.captures(&get_components()) {
        time_of_previous_sync = cap[1].parse::<u64>().unwrap();
    }

    time_of_previous_sync
}

pub fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_components(components: &Vec<(String, String)>) -> String {
    let mut formatted_components = components
        .iter()
        .map(|(x, y)| format!("{} = {}", x, y))
        .collect::<Vec<String>>()
        .join("\n");

    formatted_components.insert_str(0, &format!("[[ignition.{}]]\n", get_current_time()));
    formatted_components.push('\n');

    formatted_components
}

pub fn replace_components_in_file(formatted: String) -> String {
    let regex = Regex::new(r"(?s)\[\[ignition.\d*\]\]\n.*\[*").unwrap();
    let old_components_file = get_components();

    let new_components_file = match regex.is_match(&old_components_file) {
        true => regex.replace(&old_components_file, formatted).to_string(),
        false => formatted,
    };

    new_components_file
}

pub fn write_to_component_file(components: String) {
    if let Err(_) = components_toml().write_all(components.as_bytes()) {
        println!("Unable to write to components.toml")
    }

    if let Err(_) = fs::copy("components.toml", tempfile()) {
        println!("Unable to copy list of components to temporary file");
    }
}

pub fn components_are_locked() -> bool {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("components.lock")
        .is_err()
}

pub fn package_components_for_filing(components: &Vec<(String, String)>) -> String {
    let formatted_components = format_components(&components);
    let components_file = replace_components_in_file(formatted_components);
    eprintln!("{}", components_file);

    components_file
}

pub fn search_and_rescue_components() -> Option<Vec<(String, String)>> {
    if get_current_time() - get_time_since_last_update() > 2 {
        let components = find_components();
        let components_file = package_components_for_filing(&components);
        write_to_component_file(components_file);

        return Some(components);
    }

    None
}

pub fn update_components() -> Option<Vec<(String, String)>> {
    if !components_are_locked() {
        let components = search_and_rescue_components();
        fs::remove_file("components.lock").unwrap();

        return components;
    }

    None
}

pub fn parse_components() -> Vec<(String, String)> {
    let components = get_components();

    lazy_static! {
        static ref RE: Regex = Regex::new("(.*) = \"(.*)\"").unwrap();
    }

    RE.captures_iter(&components)
        .map(|x| (x[1].to_string(), x[2].to_string()))
        .collect::<Vec<(String, String)>>()
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

pub fn get_path_of_engine_in_current_crate() -> String {
    let regex = Regex::new(&format!(r"(?m)engine = {}()$", get_current_crate())).unwrap();
    if let Some(cap) = regex.captures(&get_components()) {
        return format!("{}{}", get_current_crate(), cap[1].to_string());
    }

    String::new()
}
