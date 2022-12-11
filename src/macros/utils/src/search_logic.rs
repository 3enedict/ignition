use std::{ffi::OsStr, fs, path::PathBuf};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::accessors::{get_component_module_path, get_module_path, source_dir};

pub fn find_components() -> Vec<(String, String)> {
    let mut components = Vec::new();
    scan_dir_for_components(&source_dir(), &mut components);

    components
}

pub fn scan_dir_for_components(dir: &PathBuf, components: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            get_components_from_file(&path, components);
        } else if path.file_name() != Some(OsStr::new("macros")) {
            scan_dir_for_components(&path, components);
        }
    }
}

pub fn get_components_from_file(path: &PathBuf, components: &mut Vec<(String, String)>) {
    /* Catches names of structs definitions similar to:
            #[derive(Debug, Conponent)] // .*#\[derive(.*Conponent.*)\].*\n
            pub struct Int { // .*struct (.*)[\{\(]
                int: i32,
            }

            #[derive(Debug, Conponent)] // .*#\[derive(.*Conponent.*)\].*\n
            pub struct Float(f32) // .*struct (.*)[\{\(]
    */
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r".*#\[derive(.*Component.*)\].*\n.*struct (.*)[\{\(]").unwrap();
    }

    let src = fs::read_to_string(path).unwrap();

    if src.contains("engine!(") {
        components.push((String::from("engine"), get_module_path(&path)));
    }

    for cap in RE.captures_iter(&src) {
        let name = get_component_name(cap);
        let module_path = get_component_module_path(&path, &name);

        components.push((name, format!("\'{}\'", module_path)));
    }
}

pub fn get_component_name(capture: Captures) -> String {
    String::from(&capture[2]).trim_matches(' ').to_string()
}

pub fn add_component_to_module_path(mut module_path: String, name: &String) -> String {
    module_path.push_str(&format!("::{{{}, {}Trait}}", name, name)); // "ignition::life::genesis::{Name, NameTrait}"
    module_path
}
