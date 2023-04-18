use std::fmt::Formatter;

#[allow(dead_code)]
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[---{}---]", self.0.join(", "))
    }
}

#[allow(dead_code)]
pub fn wra() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("{}", w);
}
