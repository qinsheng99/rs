mod cache;
mod demof;
mod directory;
mod grammar;
mod list;
mod result;
mod traits;
use clap::Parser;
use grammar::traits::same_name_trait;

#[derive(Parser)]
struct CLi {
    #[arg(short, long)]
    path: Option<String>,
}
fn main() {
    same_name_trait();
    // crate::traits::addm();

    let args = CLi::parse();
    let _p: () = match args.path {
        Some(p) => println!("{}", p),
        None => (),
    };
}
