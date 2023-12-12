use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub key: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    // 使用迭代器作为参数
    pub fn build_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let key = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 环境变量 IGNORE_CASE=1 时，忽略大小写；
        // 该环境变量不存在时，检查命令行参数
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(env) => env.eq("1"),
            Err(_) => match args.next() {
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
    contents.lines().filter(|line| line.contains(key)).collect()
}

pub fn search_case_insensitive<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&key.to_lowercase()))
        .collect()
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
