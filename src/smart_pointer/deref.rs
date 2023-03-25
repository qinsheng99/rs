use std::ops::Deref;
use std::rc::Rc;

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