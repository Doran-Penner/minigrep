use std::env;
use std::fs;

/* Notes
 * I'm doing the arg parsing differently than the guide,
 * parse_args takes ownership of args and returns true Strings
 */

struct Config {
    query: String,
    filepath: String
}

impl Config {
    fn new(mut args: env::Args) -> Self {
        let _name = args.next();
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Config { query, filepath },
            (_, _) => todo!("How should we handle missing args?"),
        }
    }
}

fn main() {
    let args = env::args();
    let config = Config::new(args);
    println!("Searching for \"{}\" in file path {}", config.query, config.filepath);

    // @graceful: file not found or badly read
    let contents = fs::read_to_string(config.filepath).expect("Couldn't read file :(");

    println!("contents of file are:\n{contents}");
}
