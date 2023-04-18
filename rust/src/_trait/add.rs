use std::fmt::Formatter;

#[allow(dead_code)]
struct Mill(i32);

impl std::ops::Add<Mill> for Mill {
    type Output = Self;

    fn add(self, o: Mill) -> Self::Output {
        Self(self.0 + 2 * o.0)
    }
}

impl std::fmt::Display for Mill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(dead_code)]
pub fn add() {
    let a = Mill(10);
    let b = Mill(20);

    println!("{}", a + b);
}
