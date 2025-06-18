use env_logger::{Builder, Env};
use std::env;

pub fn init() {
    if env::var("RUST_LOG").is_ok() {
        Builder::from_env(Env::default())
            .format_timestamp_secs()
            .init();
    } else {
        eprintln!("Warning: Logging is disabled because \"RUST_LOG=debug\" is not set.");
    }
}
