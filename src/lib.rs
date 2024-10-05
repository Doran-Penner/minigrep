use std::io::{BufRead, BufReader};
use std::{env, error, fs, io, iter, path};

struct Query {
    query_str: String,
    case_sensitive: bool,
}

impl Query {
    fn new(mut query_str: String, case_sensitive: bool) -> Self {
        if !case_sensitive {
            query_str = query_str.to_lowercase();
        };
        Query {
            query_str,
            case_sensitive,
        }
    }
    fn found_in(&self, other: &str) -> bool {
        // @lifetime: can I avoid this code duplication?
        match self.case_sensitive {
            true => other.contains(&self.query_str),
            false => {
                // shadowing variable to avoid lifetimes
                let other = &other.to_lowercase();
                other.contains(&self.query_str)
            }
        }
    }
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
                let query = Query::new(query_str, sensitive);
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
    // thank youuuuu patrick
    iter::zip(1.., reader.lines()) // add line numbers to file lines; now
        .try_for_each(|line_pair| {
            // for each line,
            // if the file reading was successful and there was a match
            if let Some(str) = matcher(line_pair, &config.query)? {
                // then print it
                println!("{}", str);
                // @perf: use io::LineWriter for **blazingly fast writing**
            };
            Ok(())
        })
}

fn matcher(
    line_pair: (u32, Result<String, io::Error>),
    query: &Query,
) -> Result<Option<String>, io::Error> {
    let (index, line_res) = line_pair;
    let line = line_res?;
    match query.found_in(&line) {
        // @feature: some way to specify formatting style
        true => Ok(Some(format!("{}: {}", index, line))),
        false => Ok(None),
    }
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
