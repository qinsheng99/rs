use std::rc::Rc;

pub mod deref;
pub mod drop;
pub mod rc;
pub mod ref_cell;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
