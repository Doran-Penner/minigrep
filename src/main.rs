use std::error::Error;
use std::{env, fs};

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn build(mut args: env::Args) -> Result<Self, Box<dyn Error>> {
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config { query, filepath }),
            (_, _) => Err("Expected two arguments, query and filepath".into()),
        }
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for \"{}\" in file path {}",
        config.query, config.filepath
    );

    let contents = fs::read_to_string(config.filepath)?;

    println!("contents of file are:\n{contents}");
    todo!()
}

// @fancy it prints the final error a bit weirdly, can it be better?
fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let config = Config::build(args)?;
    run(config)
}
