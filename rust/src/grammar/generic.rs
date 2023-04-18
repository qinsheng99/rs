#[allow(dead_code)]
pub fn generic() {
    println!("{}", add(1, 1));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.1, y: 2.2 };
    let p3 = p1.mix(p2);
    println!("{:?}", p3);

    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr = &*r;
    println!("{:?}", rr);
    r.move_data(10, 10);
    println!("{:?}", r);
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

    fn move_data(&mut self, x: T, y: U) {
        self.x = x;
        self.y = y;
    }
}

#[allow(dead_code)]
fn arr<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}
