pub trait N {
    fn _set(&mut self, _: &str);
    fn _get(&self) -> String {
        format!("{}", "impl method")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cache {
    pub path: String,
}

#[allow(dead_code)]
pub const fn new() -> Cache {
    Cache {
        path: String::new(),
    }
}

impl N for Cache {
    fn _set(&mut self, _key: &str) {
        self.path = _key.to_string()
    }

    fn _get(&self) -> String {
        self.path.clone()
    }
}

#[allow(dead_code)]
pub fn trait_type() {
    let mut c = new();
    c._set("123");
    println!("{:?}", c);

    let c1 = new();
    cache(c1, c);

    let mut c2 = impl_traut();
    c2._set("456");
    println!("{}", c2._get());
}

#[allow(dead_code)]
fn cache<T: N>(i1: T, i2: T) {
    println!("{}", i1._get());
    println!("{}", i2._get());
}
#[allow(dead_code)]
fn cache_where<T>(i1: T, i2: T)
where
    T: N,
{
    println!("{}", i1._get());
    println!("{}", i2._get());
}

#[allow(dead_code)]
fn impl_traut() -> impl N {
    Cache {
        path: String::new(),
    }
}

trait Print {
    fn print(&self) {}
}

impl Print for u8 {
    fn print(&self) {
        println!("u8:{}", self);
    }
}

impl Print for u32 {
    fn print(&self) {
        println!("u32:{}", self);
    }
}

#[allow(dead_code)]
fn p1(item: Box<dyn Print>) {
    item.print()
}

#[allow(dead_code)]
fn p2(item: &dyn Print) {
    item.print()
}

#[allow(dead_code)]
pub fn trait_object() {
    let j: u32 = 10;
    p1(Box::new(10u8));
    p2(&j)
}

#[allow(dead_code)]
pub fn same_name_trait() {
    trait A {
        fn print() -> String;
    }

    trait B {
        fn print(&self) -> String;
    }

    struct C;

    impl C {
        #[allow(dead_code)]
        fn print() -> String {
            String::from("C")
        }
    }

    impl A for C {
        fn print() -> String {
            String::from("A")
        }
    }

    impl B for C {
        fn print(&self) -> String {
            String::from("B")
        }
    }

    let c = C;
    println!("{}", C::print());
    println!("{}", B::print(&c));
    println!("{}", <C as A>::print());
}

//定义一个实现area的trait
pub trait HH {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl HH for Circle {
    //定义结构体实现area
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
#[allow(dead_code)]
pub fn trais_demo() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 2.0,
    };
    println!("circle c has an area of {}", c.area());
}

trait Foo {
    fn foo(&self);
}

trait FooBar: Foo {
    fn foobar(&self);
}
struct Baz;

impl Foo for Baz {
    fn foo(&self) {
        println!("foo");
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
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
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

#[derive(Debug)]
struct Point1<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add trait，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point1<T> {
    type Output = Point1<T>;

    fn add(self, p: Point1<T>) -> Point1<T> {
        Point1 {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

trait Method {
    fn method(&self) -> String;
}

impl Method for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Method for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

#[allow(dead_code)]
fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

#[allow(dead_code)]
pub fn method() {
    let x = "Hello".to_string();
    println!("x--{}", x.method());
    let y: u8 = 8;
    println!("y--{}", y.method());

    println!("5 + 6 = {}", add(5, 6));
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[allow(dead_code)]
pub fn addm() {
    let a = Millimeters(10);
    let b = Meters(20);
    let c = a + b;
    println!("{:?}", c);
}

