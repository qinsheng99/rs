use std::{error::Error, fs};

#[test]
fn test() {
    assert_eq!(1, 1)
}
pub struct Config {
    pub text: String,
    pub path: String,
}

#[allow(dead_code)]
pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;
    Ok(content)
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args err");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            text: query,
            path: filename,
        })
    }
}
