use std::error::Error;
use std::{env, fs};
use std::{env, fs};

pub struct Config {
    pub key: String,
    pub file: String,
    pub ignore_case: bool,
    pub ignore_case: bool,
}

impl Config {
    pub fn build_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let key = args[1].clone();
        let file = args[2].clone();

        // 环境变量 IGNORE_CASE=1 时，忽略大小写；
        // 该环境变量不存在时，检查命令行参数
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(env) => env.eq("1"),
            Err(_) => match args.get(3) {
                Some(arg) => arg.eq("ignore_case"),
                None => false,
            },
        };

        Ok(Config {
            key,
            file,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    // println!("contents:");
    // print!("{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.key, &contents)
    } else {
        search(&config.key, &contents)
    };

    println!("Results:{:?}", results);
    Ok(())
}

pub fn search<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    // println!("contents:");
    // print!("{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.key, &contents)
    } else {
        search(&config.key, &contents)
    };

    println!("Results:{:?}", results);
    Ok(())
}

pub fn search<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(key) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let key = key.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&key) {
        if line.contains(key) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let key = key.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&key) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
