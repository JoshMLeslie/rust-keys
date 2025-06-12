use simplelog::*;
use std::fs::File;

/* Example usage:
fn main() {
    // Initialize the logger once at startup
    Logger::init();

    // use the info! macro directly
    debug!("Direct logging tada");

		// observe updates in another terminal via tail
		tail -f debug.log
}
*/
pub struct Logger;

impl Logger {
    pub fn init() {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("debug.log").unwrap(),
        )
        .unwrap();
    }
}
