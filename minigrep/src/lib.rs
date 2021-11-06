use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
test";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}




