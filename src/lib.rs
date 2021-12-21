pub mod renderer;
pub mod objects;

#[cfg(all(debug_assertions))]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;
