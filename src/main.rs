use std::env;
use std::process;

use clap::Parser;

#[allow(unused_imports)]
use grammar::life_cycle;
use rust::Config;

mod cache;
mod demof;
mod directory;
mod grammar;
mod list;
mod result;

#[derive(Parser)]
struct CLi {
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    // life_cycle();

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("err is {}", err);
        process::exit(1);
    });

    // if let Err(err) = run(config) {
    //     println!("err is {:?}", err);
    //     process::exit(1);
    // }

    let content = rust::run(config).unwrap_or_else(|err| {
        println!("read file failed, err is {:?}", err);
        process::exit(1);
    });

    println!("{}", content);
}

#[allow(dead_code)]
fn args() {
    // let args = CLi::parse();
    // let _p: () = match args.path {
    //     Some(p) => println!("{}", p),
    //     None => (),
    // };
}
