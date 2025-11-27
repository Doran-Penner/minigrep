use std::io::{BufRead, BufReader, LineWriter, Write};
use std::{env, error, fs, io, iter, path};

/* goals:
 * if one file (including stdin), just do that
 * if directory... print each file name?
 * (can we get bat-like functionality?)
 */

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
                let other = other.to_lowercase();
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
        let query_str = args.next().ok_or("Expected first argument, query")?;
        // @feature (crazy): make it flexibly work with stdin or given filepath
        let filepath_str = args.next().ok_or("Expected second argument, filepath")?;
        let query = Query::new(query_str, sensitive);
        let filepath = path::PathBuf::from(filepath_str);
        match filepath.try_exists() {
            Ok(true) => Ok(Config { query, filepath }),
            _ => Err("Could not access file".into()),
        }
    }
}

fn matcher(index: u32, line: String, query: &Query) -> Option<String> {
    match query.found_in(&line) {
        // @feature: some way to specify formatting style
        true => Some(format!("{}: {}\n", index, line)),
        false => None,
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(&config.filepath)?;
    let reader = BufReader::new(file);
    let mut writer = LineWriter::new(io::stdout());
    // thank youuuuu patrick
    iter::zip(1.., reader.lines()) // add line numbers to file lines; now
        .try_for_each(|(index, line_res)| {
            // for each line,
            // if the file reading was successful
            let line = line_res?;
            // and there was a match
            if let Some(str) = matcher(index, line, &config.query) {
                // then print it
                writer.write_all(str.as_bytes())?;
            };
            Ok(())
        })
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
