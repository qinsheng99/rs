pub mod compound_type;
pub mod form;
pub mod format;
pub mod generic;
pub mod json;
pub mod life_cycle;
pub mod newtype;
pub mod static_gra;
pub mod traits;
pub mod variable;
pub mod vec;
pub mod iter;
use life_cycle::max_str;
use static_gra::static_ as s;

#[allow(dead_code)]
pub fn life_cycle() {
    let a = "hello"; //life 13-25
    let b = String::from("world"); //life 14-25
    let c = max_str(a, b.as_str()); //取b的life 14-25  包含c的life 15-25
    println!("{}", c);

    // let aa = "hello"; //life 8-15
    // let cc;
    // {
    //     let bb = String::from("world"); // life 11-13
    //     cc = max_str(aa, bb.as_str()); // life 12-13  cc:9-15
    // }
    // println!("{}", cc); //build err
}

#[allow(dead_code)]
pub fn static_() {
    s()
}
