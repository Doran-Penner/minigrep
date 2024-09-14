use std::error::Error;
use std::fs;

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
    let contents = fs::read_to_string(config.filepath)?;
    todo!("{}", contents)
}

// @optimize: best type of string to pass around?
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".to_string();
        assert_eq!(vec!["safe, fast, productive."], search("duct", &contents));
        assert_eq!(vec!["safe, fast, productive."], search("safe", &contents));
        assert_eq!(vec!["Rust:"], search("R", &contents));
        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(".", &contents));
        assert_eq!(vec!["Rust:", "safe, fast, productive.", "Pick three."], search("t", &contents));
    }
}
