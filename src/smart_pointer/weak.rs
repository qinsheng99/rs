use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[allow(dead_code)]
pub fn weak() {
    let a = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("{:?}", a.parent.borrow().upgrade());
    println!("a strong count {}", Rc::strong_count(&a));
    println!("a weak count {}", Rc::weak_count(&a));

    let b = Rc::new(Node {
        value: 20,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&a)]),
    });

    *a.parent.borrow_mut() = Rc::downgrade(&b);
    println!("b strong count {}", Rc::strong_count(&b));
    println!("b weak count {}", Rc::weak_count(&b));
    // println!("{:?}", a.parent.borrow().upgrade());
}
