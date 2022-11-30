mod cache;
mod demof;
mod directory;
mod grammar;
mod list;
mod result;
mod traits;
use crate::traits::trais_demo;
use clap::Parser;
use grammar::generic::generic;

#[derive(Parser)]
struct CLi {
    #[arg(short, long)]
    path: Option<String>,
}
fn main() {
    generic();

    let args = CLi::parse();
    let _p: () = match args.path {
        Some(p) => println!("{}", p),
        None => (),
    };

    trais_demo()
}
