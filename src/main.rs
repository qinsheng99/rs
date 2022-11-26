mod cache;
mod traits;
mod directory;
mod demof;
mod list;
mod result;
mod grammar;
use grammar::{compound_type::compound_type_practice};
use clap::{Parser};
use crate::traits::N;

#[derive(Parser)]
struct CLi {
   #[arg(short, long)]
   path:Option<String>
}
fn main() {
   compound_type_practice();

   let args = CLi::parse();
   let _p:() = match args.path {
      Some(p) => println!("{}", p),
      None => ()
   };

   // let mut c = traits::new();
   //
   // c._set("zbc");

   crate::traits::method();

}



