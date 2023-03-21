use std::{env, error::Error, fs};

#[test]
fn test() {
    assert_eq!(1, 1)
}

pub struct Config {
    pub text: String,
    pub path: String,
    pub case: bool,
}

#[allow(dead_code)]
pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;
    let result = if config.case {
        case_search(&config.text, &content)
    } else {
        search(&config.text, &content)
    };
    for l in result {
        println!("{}", l)
    }
    Ok(content)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut v: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         v.push(line)
    //     }
    // }
    // v

    contents.lines().filter(|x| x.contains(query)).collect()
}

pub fn case_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut v: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         v.push(line)
    //     }
    // }
    // v

    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query))
        .collect()
}

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("args err");
//         }
//
//         let query: String = args[1].clone();
//         let filename: String = args[2].clone();
//         let case = env::var("CASE").is_ok();
//         Ok(Config {
//             text: query,
//             path: filename,
//             case,
//         })
//     }
// }

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args err");
        }
        args.next();

        let query: String = args.next().unwrap();
        let filename: String = args.next().unwrap();

        let case = env::var("DB_HOST").is_ok();
        Ok(Config {
            text: query,
            path: filename,
            case,
        })
    }
}
