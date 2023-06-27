pub trait Iter {
    type Item;

    fn i() -> Option<Self::Item>;
}

impl Iter for A {
    type Item = u32;

    fn i() -> Option<Self::Item> {
        Some(3)
    }
}

struct A;
pub trait Fully {
    fn i() -> String;
}

impl A {
    fn i() -> String {
        String::from("A")
    }
}

impl Fully for A {
    fn i() -> String {
        String::from("Fully")
    }
}

pub fn f() {
    println!("{}", A::i());
    println!("{}", <A as Fully>::i());
}

trait B: std::fmt::Display {}
