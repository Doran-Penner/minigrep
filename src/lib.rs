use std::error::Error;
use std::{fs, iter};

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config { query, filepath }),
            (_, _) => Err("Expected two arguments, query and filepath".into()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    for (line_num, line) in search(&config.query, &contents) {
        println!("{}: {}", line_num, line);
    }

    Ok(())
}

// @optimize: best type of string to pass around?
pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = (u32, &'a str)> + 'a {
    // @learn I'm not sure how I feel about the closure owning query, what does that actually mean?
    // does it take it from the external Config or just this function's input?
    // Surely this one, since query was passed in as an immutable reference
    iter::zip(1.., contents.lines()).filter(move |(_, line)| line.contains(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three."
            .to_string();
        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search("duct", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search("safe", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(1, "Rust:")],
            search("R", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(2, "safe, fast, productive."), (3, "Pick three.")],
            search(".", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                (1, "Rust:"),
                (2, "safe, fast, productive."),
                (3, "Pick three.")
            ],
            search("t", &contents).collect::<Vec<_>>()
        );
    }

    // @production: I could add more tests for each thing, or extract the read_to_string from run,
    // or other things
}
