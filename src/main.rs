use std::env;
use std::process;

use clap::Parser;

#[allow(unused_imports)]
use grammar::compound_type::debug;
#[allow(unused_imports)]
use grammar::life_cycle;
#[allow(unused_imports)]
use grammar::life_cycle::life;
#[allow(unused_imports)]
use grammar::static_;
use rust::Config;

mod _trait;
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
    // static_();
    // debug()
    // args()

    // _trait::add::add()

    grammar::json::json_unmarshal();
    grammar::json::json_marshal()
}

#[allow(dead_code)]
fn args() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("err is {}", err);
        process::exit(1);
    });

    if let Err(err) = rust::run(config) {
        println!("err is {:?}", err);
        process::exit(1);
    }
}

#[allow(dead_code)]
fn cli() {
    let args = CLi::parse();
    let _p: () = match args.path {
        Some(p) => println!("{}", p),
        None => (),
    };
}
