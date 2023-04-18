#[allow(dead_code)]
#[warn(unused_variables)]
fn pattern() {
    let _a: () = match std::fs::read_to_string("vec.rs") {
        Ok(_s) => (),
        Err(_e) => (),
    };

    if let Ok(s) = std::fs::read_to_string("vec.rs") {
        println!("{}", s)
    }

    while let Some(s) = vec![1, 2, 3].pop() {
        println!("{}", s)
    }

    for (k, v) in vec![1, 2, 3].iter().enumerate() {
        println!("{}-{}", k, v);
    }

    let (_x, _y) = (1, 2);

    fn pa(&(x, y): &(i32, i32)) {
        println!("{}-{}", x, y);
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };

    let Point { x, y } = p;
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}
