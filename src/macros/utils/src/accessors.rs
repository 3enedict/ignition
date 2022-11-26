use std::{env, fs, fs::OpenOptions, path::PathBuf, time::SystemTime};

use regex::Regex;

use crate::search_logic::add_component_to_module_path;

/* FILES */

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

pub fn get_components() -> String {
    match fs::read_to_string("components.toml") {
        Ok(ids) => ids,
        Err(_) => match fs::read_to_string(tempfile().as_path()) {
            Ok(ids) => ids,
            Err(_) => String::new(),
        },
    }
}

/* PATHS */

pub fn source_dir() -> PathBuf {
    let mut source_directory = PathBuf::new();
    source_directory.push(".");
    source_directory.push("src");

    source_directory
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

/* INFORMATION */

pub fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_time_since_last_update() -> u64 {
    let regex = Regex::new(r"\[\[ignition.(\d*)\]\]").unwrap();

    let mut time_of_previous_sync = 0;
    if let Some(cap) = regex.captures(&get_components()) {
        time_of_previous_sync = cap[1].parse::<u64>().unwrap();
    }

    time_of_previous_sync
}

pub fn components_are_locked() -> bool {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("components.lock")
        .is_err()
}
