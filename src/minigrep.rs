use std::error::Error;
use std::fs;

pub struct Config {
    pub key: String,
    pub file: String,
}

impl Config {
    pub fn build_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let key = args[1].clone();
        let file = args[2].clone();

        Ok(Config { key, file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("contents:");
    print!("{contents}");

    let key = config.key;
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(&key) {
            results.push(line);
        }
    }
    println!("Results:{:?}", results);

    Ok(())
}
