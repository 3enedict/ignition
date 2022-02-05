//TODO: Generate pipeline using renderer object for exterior use

pub mod core;

#[cfg(all(debug_assertions))]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;
