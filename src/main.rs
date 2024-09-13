use std::{ env, fs, process };

/* Notes
 * I'm doing the arg parsing differently than the guide,
 * parse_args takes ownership of args and returns true Strings
 */

struct Config {
    query: String,
    filepath: String
}

impl Config {
    fn build(mut args: env::Args) -> Result<Self, &'static str> {
        let _name = args.next();
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config { query, filepath }),
            (_, _) => Err("expected 2 arguments, query and filepath"),
        }
    }
}

fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|msg| {
        println!("Error parsing config: {}", msg);
        process::exit(1);
    });
    println!("Searching for \"{}\" in file path {}", config.query, config.filepath);

    // @graceful: file not found or badly read
    let contents = fs::read_to_string(config.filepath).expect("Couldn't read file :(");

    println!("contents of file are:\n{contents}");
}
