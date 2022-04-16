pub fn plus_one() {
    let plus = |x :i64| x + 2;
    println!("plus---{}", plus(3));
}

pub fn closure_1() {
    let nums = vec![1, 2, 3];

    let takes_nums = || nums;

    println!("{:?}", takes_nums());
}

pub fn closure_2(){

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
    
        add_num(5);
    }
    println!("num = {}", num)
}

pub fn closure_3(){

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(4);
    }
    println!("num = {}", num)
}