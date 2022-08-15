#[allow(dead_code)]
pub fn plus_one() {
    let plus = |x :i64| x + 2;
    println!("plus---{}", plus(3));
}
#[allow(dead_code)]
pub fn closure_1() {
    let nums = vec![1, 2, 3];

    let takes_nums = || nums;

    println!("{:?}", takes_nums());
}
#[allow(dead_code)]
pub fn closure_2(){

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
    
        add_num(5);
    }
    println!("num = {}", num)
}
// #[allow(dead_code)]
// pub fn closure_3(){
//
//     let mut num = 5;
//     {
//         let mut add_num = move |x: i32| num += x;
//         add_num(4);
//     }
//     println!("num = {}", num)
// }

#[allow(dead_code)]
pub fn closure_4() {
    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {

        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    assert_eq!(4, answer);
}

#[allow(dead_code)]
pub fn closure_5() {
    fn call_with_one(some_closure: &dyn Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(&|x| x + 2);

    assert_eq!(3, answer);
}