pub mod compound_type;
pub mod form;
pub mod format;
pub mod generic;
mod life_cycle;
pub mod traits;
pub mod variable;
pub mod vec;
use life_cycle::max_str;

#[allow(dead_code)]
pub fn life_cycle() {
    let a = "hello"; //life 3-7
    let b = String::from("world"); //life 4-7
    let c = max_str(a, b.as_str()); //取b的life 4-7  包含c的life 5-7
    println!("{}", c);

    // let aa = "hello"; //life 8-15
    // let cc;
    // {
    //     let bb = String::from("world"); // life 11-13
    //     cc = max_str(aa, bb.as_str()); // life 12-13  cc:9-15
    // }
    // println!("{}", cc); //build err
}
