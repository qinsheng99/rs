pub mod add;
pub mod debug;
pub mod deref;
pub mod display;

#[allow(dead_code)]
struct Example {
    pub id: String,
}

trait New {
    fn set(&mut self, s: &str);
}

const fn new() -> Example {
    Example { id: String::new() }
}

impl New for Example {
    fn set(&mut self, s: &str) {
        self.id = s.to_string()
    }
}
