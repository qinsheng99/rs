pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[allow(dead_code)]
pub fn r#box() {
    let x = String::from("5");
    let y = MyBox::new(x);
    assert_eq!("5", *y);

    hello(&y);
    // &y &MyBox<String>
    // &String &str
}

#[allow(dead_code)]
fn hello(name: &str) {
    println!("Hello {}", name)
}
