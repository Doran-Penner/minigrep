use std::error::Error;
use std::{fs, iter, env};

pub struct Config {
    query: String,
    filepath: String,
    sensitive: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        let sensitive = env::var("IGNORE_CASE").is_err();
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config { query, filepath, sensitive }),
            (_, _) => Err("Expected two arguments, query and filepath".into()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    // @ugly: can't assigne var `matches` to output of search_ because
    // they have different output types (silly returning iterator)
    if config.sensitive {
        for (line_num, line) in search_sens(&config.query, &contents) {
            println!("{}: {}", line_num, line);
        }
    } else {
        for (line_num, line) in search_insens(&config.query, &contents) {
            println!("{}: {}", line_num, line);
        }
    }

    Ok(())
}

pub fn search_sens<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = (u32, &'a str)> + 'a {
    iter::zip(1.., contents.lines()).filter(move |(_, line)| line.contains(query))
}

pub fn search_insens<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = (u32, &'a str)> + 'a {
    iter::zip(1.., contents.lines()).filter(move |(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
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
            search_sens("duct", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search_sens("safe", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(1, "Rust:")],
            search_sens("R", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(2, "safe, fast, productive."), (3, "Pick three.")],
            search_sens(".", &contents).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                (1, "Rust:"),
                (2, "safe, fast, productive."),
                (3, "Pick three.")
            ],
            search_sens("t", &contents).collect::<Vec<_>>()
        );
    }

    // @production: I could add more tests for each thing, or extract the read_to_string from run,
    // or other things
}
