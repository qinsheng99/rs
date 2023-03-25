use std::rc::Rc;

use crate::smart_pointer::List::{Cons, Nil};

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
