use std::fs;

use regex::Regex;

pub mod accessors;
pub mod parsing;
pub mod search_logic;

use crate::{
    accessors::{
        components_are_locked, get_components, get_current_crate, get_current_time,
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

        let components = components.into_iter().filter(|(_x, y)| y.contains("'"));
        return Some(components.collect());
    }

    None
}

pub fn package_components_for_filing(components: &Vec<(String, String)>) -> String {
    let formatted_components = format_components(&components);
    let components_file = replace_components_in_file(formatted_components);

    components_file
}

pub fn format_components(components: &Vec<(String, String)>) -> String {
    let mut formatted_components = components
        .iter()
        .map(|(x, y)| format!("{} = {}", x, y))
        .collect::<Vec<String>>()
        .join("\n");

    let crate_name = get_current_crate();
    let time = get_current_time();

    formatted_components.insert_str(0, &format!("[[{}.{}]]\n", crate_name, time));
    formatted_components.push('\n');

    formatted_components
}

pub fn replace_components_in_file(formatted: String) -> String {
    let old_components_file = get_components();

    if old_components_file.is_empty() {
        return formatted;
    }

    let cur_crate = get_current_crate();
    let regex = Regex::new(&format!(r"\[\[{}.\d*\]\]\n(?:.* = .*\n)*", cur_crate)).unwrap();

    let new_components_file = match regex.is_match(&old_components_file) {
        true => regex.replace(&old_components_file, formatted).to_string(),
        false => old_components_file + "\n" + &formatted,
    };

    new_components_file
}

pub fn write_to_component_file(components: String) {
    if let Err(_) = fs::write("components.toml", components) {
        println!("Unable to write to components.toml")
    }

    if let Err(_) = fs::copy("components.toml", tempfile()) {
        println!("Unable to copy list of components to temporary file");
    }
}
