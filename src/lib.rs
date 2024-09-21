use std::io::{BufRead, BufReader};
use std::{env, error, fs, io, iter, path};

enum Query {
    CaseSens(String),
    CaseInsens(String),
}

pub struct Config {
    query: Query,
    // @learn: can I avoid a box?
    filepath: Box<path::Path>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn error::Error>> {
        // @fancy: use args_os in main and change code accordingly
        let sensitive = env::var("IGNORE_CASE").is_err();
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query_str), Some(filepath_str)) => {
                let query = if sensitive {
                    Query::CaseSens(query_str)
                } else {
                    Query::CaseInsens(query_str.to_lowercase())
                };
                let filepath: Box<path::Path> = path::Path::new(&filepath_str).into();
                // @fancy: use filepath.exists or filepath.try_exists for better error
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
    let lines = reader.lines();
    let with_linenums = iter::zip(1.., lines);
    for line in with_linenums {
        handler(line, &config.query)?;
    }

    Ok(())
}

fn handler(line: (u32, Result<String, io::Error>), query: &Query) -> Result<(), io::Error> {
    // @clean: better var names (they degrade around here)
    let (idx, val_result) = line;
    let mut val = val_result?;
    let query_str = match query {
        Query::CaseSens(q) => q,
        Query::CaseInsens(q) => {
            val = val.to_lowercase();
            q
        }
    };
    if val.contains(query_str) {
        println!("{}: {}", idx, val);
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
