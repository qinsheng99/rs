
const MAX: u32 = 100_000;
#[allow(dead_code)]
pub fn variable_demo(){
    let f = "hello";
    println!("{}", f);
    println!("{}", MAX);

    let x = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!("{}", x)
    }
    println!("{}", x);
    
}