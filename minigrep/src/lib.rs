use std::error::Error;
use std::{env, fs};

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    let results = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_case_insensetive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line)
    }
    Ok(())
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|&line| line.contains(query)).collect()
}
fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|&line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

pub struct Config {
    query: String,
    path: String,
    case_sensetive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str>  {
        if args.len() < 3 {
            return Err("Not enough arguments")
        };
        let case_sensetive = env::var("CASE_INSENSETIVE").is_err();
        Ok(Config {query: args[1].clone(), path: args[2].clone(), case_sensetive})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\

Rust:
Safe, fast, productive.
Pick three.";
        assert_eq!(search(query, contents), vec!["Safe, fast, productive."]);
    }

    #[test]
    fn case_test(){
        let query = "rust";
        let contents = "\

Rust:
Safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(search_case_insensetive(query, contents), vec!["Rust:", "Trust me"]);
        assert_eq!(search(query, contents), vec!["Trust me"]);
    }
}