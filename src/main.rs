mod cache;
mod cache1;
mod english;
mod traits;
// use std::io::Write;

use english::farewells;
use cache1::cache2::d;
use crate::traits::HH;
fn main() {
    // println!("Hello in English: {}", farewells::fi());

    // let mut c = cache::Cache::new();
    // let s = String::from("_");
    // c.set(s);
    // c.set(String::from(""));

    println!("{}", farewells::fi());
    d::dd();

    let a = [1,2,3,4,5,6,7];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(d::get_func(*i)(*i));
    }
    println!("{:?}", b);

    let x = 1;
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c),
    }

    println!("x: {}", x);

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_)}) => println!("{:?}", a),
        _ => {}
    }

    let c = traits::Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    println!("circle c has an area of {}", c.area());
    
    
    let x = "Hello".to_string();
    println!("x--{}", x.method());
    let y = 8u8;
    println!("y--{}", y.method());

    println!("5 + 6 = {}", add(5, 6));
    
}
trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

use std::ops::Add;

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}



