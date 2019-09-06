use std::env;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}


// Box<Error> means that the function will return a type that implements Err#
// but we don't need to specify that particular type.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? at the end is to return error value from from the current function
    // for caller to handle.
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("Not enough arguments.");
        }
        // args is owned by main, if we assign them here to query and filena#
        // that will be in violation to the borrow rules in rustlang
        // So, the easiest way for now is to make full copy o fthe args to be
        // stored in Config struct's fields. This is inefficient though but
        // helps avoiding the problem mentioned above by allowing us to not #
        // concerned about lifetimes of these variables.
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // to_lowercase allocates a new var of type String
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        // contains takes in a string slice hence the ampersand
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "end";
        let content = "\
    This is not the end.
    This is not the beginning.
    Mike Shinoda
        ";
        assert_eq!(
            vec!["This is not the end."],
            search_case_sensitive(query, content)
            );
    }

    #[test]
    fn case_insensitive() {
        let query = "End";
        let content = "\
    This is not the end.
    This is not the beginning.
    Mike Shinoda
        ";
        assert_eq!(
            vec!["This is not the end."],
            search_case_insensitive(query, content)
            );
    }
}
