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
