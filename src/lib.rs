use std::{env, error, fs, iter};

pub struct Config {
    query: String,
    filepath: String,
    sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn error::Error>> {
        let sensitive = env::var("IGNORE_CASE").is_err();
        args.next(); // gets rid of executable name
        let query_opt = args.next();
        let filepath_opt = args.next();
        match (query_opt, filepath_opt) {
            (Some(query), Some(filepath)) => Ok(Config {
                query,
                filepath,
                sensitive,
            }),
            (_, _) => Err("Expected two arguments, query and filepath".into()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    for (line_num, line) in search(&config.query, &contents, config.sensitive) {
        println!("{}: {}", line_num, line);
    }

    Ok(())
}

pub fn search<'a>(
    query: &'a str,
    contents: &'a str,
    case_sensitive: bool,
) -> impl Iterator<Item = (u32, &'a str)> + 'a {
    iter::zip(1.., contents.lines()).filter(move |(_, line)| {
        (if case_sensitive {
            line.to_string()
        } else {
            line.to_lowercase()
        })
        .contains(
            &(if case_sensitive {
                query.to_string()
            } else {
                query.to_lowercase()
            }),
        )
    })
}

#[cfg(test)]
mod tests {
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

    // @production: break this test up, add tests for other cases and other functions
}
