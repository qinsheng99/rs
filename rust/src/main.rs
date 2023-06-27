use std::env;
use std::process;

use clap::Parser;

#[allow(unused_imports)]
use grammar::compound_type::debug;
#[allow(unused_imports)]
use grammar::iter::shoes;
#[allow(unused_imports)]
use grammar::life_cycle;
#[allow(unused_imports)]
use grammar::life_cycle::life;
#[allow(unused_imports)]
use grammar::static_;
use rust::Config;

mod _macro;
mod _trait;
mod cache;
mod demof;
mod directory;
mod grammar;
mod list;
mod net_http;
mod result;
mod smart_pointer;
mod thread;
mod un_safe;

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
    // fn_()

    // shoes()

    // grammar::iter::iter()
    // args()
    // thread::mutex::threds_metux()

    // let a = std::time::Duration::from_secs(10).as_secs();
    // println!("err is {}", 1);

    // net_http::server::ser()

    // un_safe::unsafe_split()

    _macro::v();

    _trait::advanced_trait::f()
}

#[allow(dead_code)]
fn args() {
    // let args: Vec<String> = env::args().collect();
    //
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("err is {}", err);
    //     process::exit(1);
    // });
    //
    // if let Err(err) = rust::run(config) {
    //     println!("err is {:?}", err);
    //     process::exit(1);
    // }

    let config = Config::new(env::args()).unwrap_or_else(|err| {
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
