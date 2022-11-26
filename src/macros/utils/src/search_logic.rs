use std::{ffi::OsStr, fs, path::PathBuf};

use regex::{Captures, Regex};

use crate::accessors::{get_component_module_path, get_module_path, source_dir};

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

pub fn add_component_to_module_path(mut module_path: String, name: &String) -> String {
    module_path.push_str(&format!("::{{{}, {}Trait}}", name, name)); // "ignition::life::genesis::{Name, NameTrait}"
    module_path
}
