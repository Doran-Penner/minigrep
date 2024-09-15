use minigrep::{run, Config};
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args();
    let config = Config::build(args)?;
    run(config)
}
