use crate::_trait::new;
use crate::_trait::{Example, New};
use std::fmt::{Debug, Formatter};

impl Debug for Example {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("id", &self.id)
            .field("id1", &self.id)
            .finish()
    }
}

#[allow(dead_code)]
pub fn debug() {
    let mut ex = new();
    let s = "123";
    ex.set(s);

    println!("{:?}", ex)
}
