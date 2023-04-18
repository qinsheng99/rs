use crate::_trait::new;
use crate::_trait::{Example, New};
use std::fmt::Formatter;

impl std::fmt::Display for Example {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-{}-", self.id)
    }
}

#[allow(dead_code)]
pub fn dis() {
    let mut ex = new();
    let s = "123";
    ex.set(s);

    println!("{}", format!("{}", ex))
}
