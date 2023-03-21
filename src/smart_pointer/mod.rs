use crate::smart_pointer::List::{Cons, Nil};
use std::ops::Deref;
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
    Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
