#[allow(dead_code)]
pub fn generic() {
    println!("{}", add(1, 1));
}
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
