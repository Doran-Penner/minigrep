use std::io::{BufRead, BufReader};
use std::{env, error, fs, io, iter, path};

enum Query {
    CaseSens(String),
    CaseInsens(String),
}

pub struct Config {
    query: Query,
    filepath: path::PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn error::Error>> {
        let sensitive = env::var("IGNORE_CASE").is_err();
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query_str), Some(filepath_str)) => {
                let query = match sensitive {
                    true => Query::CaseSens(query_str),
                    false => Query::CaseInsens(query_str.to_lowercase()),
                };
                let filepath = path::PathBuf::from(filepath_str);
                match filepath.exists() {
                    true => Ok(Config { query, filepath }),
                    false => Err("Could not access file".into()),
                }
            }
            (_, _) => Err("Expected two arguments, query and filepath".into()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(&config.filepath)?;
    let reader = BufReader::new(file);
    let lines_with_linenums = iter::zip(1.., reader.lines());
    for line_pair in lines_with_linenums {
        printer(line_pair, &config.query)?;
    }

    Ok(())
}

fn printer(line_pair: (u32, Result<String, io::Error>), query: &Query) -> Result<(), io::Error> {
    let (index, line_res) = line_pair;
    let mut line = line_res?;
    let query_str = match query {
        Query::CaseSens(q) => q,
        Query::CaseInsens(q) => {
            line = line.to_lowercase();
            q
        }
    };
    if line.contains(query_str) {
        println!("{}: {}", index, line);
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    /*
        use super::*;

        #[test]
        fn test_search_sens() {
            let contents = "\
    Rust:
    safe, fast, productive.
    Pick three."
                .to_string();
            assert_eq!(
                vec![(2, "safe, fast, productive.")],
                search("duct", &contents, true).collect::<Vec<_>>()
            );
            assert_eq!(
                vec![(2, "safe, fast, productive.")],
                search("safe", &contents, true).collect::<Vec<_>>()
            );
            assert_eq!(
                vec![(1, "Rust:")],
                search("R", &contents, true).collect::<Vec<_>>()
            );
            assert_eq!(
                vec![(2, "safe, fast, productive."), (3, "Pick three.")],
                search(".", &contents, true).collect::<Vec<_>>()
            );
            assert_eq!(
                vec![
                    (1, "Rust:"),
                    (2, "safe, fast, productive."),
                    (3, "Pick three.")
                ],
                search("t", &contents, true).collect::<Vec<_>>()
            );
        }

        */
    // @todo: actually make tests!
}
