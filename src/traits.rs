//定义一个实现area的trait
pub trait HH {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl HH for Circle {//定义结构体实现area
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub fn trais_demo() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    println!("circle c has an area of {}", c.area());
}

trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}
struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
use std::ops::Add;
// 为Point实现Add trait
impl Add for Point {
    type Output = Point; //执行返回值类型为Point
    fn add(self, p: Point) -> Point {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}
// let p1 = Point{x: 1, y: 1};
// let p2 = Point{x: 2, y: 2};
// println!("{:?}", add(p1+p2, p2));

#[derive(Debug)]
struct Point1<T: Add<T, Output = T>> { //限制类型T必须实现了Add trait，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point1<T> {
    type Output = Point1<T>;

    fn add(self, p: Point1<T>) -> Point1<T> {
        Point1{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

// trait Foo { fn method(&self) -> String; }

// impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
// impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

// use std::ops::Add;

// fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
//     a + b
// }

// let x = "Hello".to_string();
//     println!("x--{}", x.method());
//     let y = 8u8;
//     println!("y--{}", y.method());

//     println!("5 + 6 = {}", add(5, 6));