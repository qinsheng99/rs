use std::ops::Deref;
use std::rc::Rc;

use crate::smart_pointer::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
pub fn r#box() {
    let x = String::from("5");
    let y = MyBox::new(x);
    assert_eq!("5", *y);

    hello(&y);
    // &y &MyBox<String>
    // &String &str
}

#[allow(dead_code)]
fn hello(name: &str) {
    println!("Hello {}", name)
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[allow(dead_code)]
struct DropSmartPoint {
    data: String,
}

impl Drop for DropSmartPoint {
    fn drop(&mut self) {
        println!("{}", self.data)
    }
}

#[allow(dead_code)]
pub fn dbox() {
    let _a = DropSmartPoint {
        data: String::from("123"),
    };

    drop(_a);
    let _b = DropSmartPoint {
        data: String::from("456"),
    };

    println!("create")
}

#[allow(dead_code)]
pub fn rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("{}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }

    println!("{}", Rc::strong_count(&a));
}
