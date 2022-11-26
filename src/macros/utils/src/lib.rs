use std::{fs, io::prelude::*};

use regex::Regex;

pub mod accessors;
pub mod parsing;
pub mod search_logic;

use crate::{
    accessors::{
        components_are_locked, components_toml, get_components, get_current_time,
        get_time_since_last_update, tempfile,
    },
    search_logic::find_components,
};

pub fn update_components() -> Option<Vec<(String, String)>> {
    if !components_are_locked() {
        let components = search_and_rescue_components();
        fs::remove_file("components.lock").unwrap();

        return components;
    }

    None
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

pub fn package_components_for_filing(components: &Vec<(String, String)>) -> String {
    let formatted_components = format_components(&components);
    let components_file = replace_components_in_file(formatted_components);
    eprintln!("{}", components_file);

    components_file
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
