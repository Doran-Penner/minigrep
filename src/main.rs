use minigrep::{Config, run};
use std::env;
use std::error::Error;

// @fancy it prints the final error a bit weirdly, can it be better?
fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let config = Config::build(args)?;
    run(config)
}
