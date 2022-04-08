pub fn plus_one() {
    let plus = |x :i64| x + 2;
    println!("plus---{}", plus(3));
}

pub fn closure_1() {
    let nums = vec![1, 2, 3];

    let takes_nums = || nums;

    println!("{:?}", takes_nums());
}