pub mod prelude;

pub fn logger() {
    if env_logger::try_init().is_err() {
        println!("Warning: Unable to start logger. This may be because it has already been started, especially during tests.");
    }
}

pub struct Engine {}

impl Engine {
    pub fn ignite() -> Self {
        logger();

        Self {}
    }
}
