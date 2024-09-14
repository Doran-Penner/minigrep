use std::{env, fs, io};

/* Notes
 * I'm doing the arg parsing differently than the guide,
 * parse_args takes ownership of args and returns true Strings
 */

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn build(mut args: env::Args) -> Result<Self, io::Error> {
        args.next(); // get rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config { query, filepath }),
            // @detail should it be a String? Rust was happy to convert it before...
            (_, _) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected two arguments, query and filepath",
            )),
        }
    }
}

// @fancy it prints the final error a bit weirdly, what's the best way to do it?
fn main() -> Result<(), io::Error> {
    let args = env::args();
    let config = Config::build(args)?;
    println!(
        "Searching for \"{}\" in file path {}",
        config.query, config.filepath
    );

    // @graceful: file not found or badly read
    let contents = fs::read_to_string(config.filepath)?;

    println!("contents of file are:\n{contents}");

    Ok(())
}
