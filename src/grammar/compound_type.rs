#[allow(dead_code)]
pub fn compound_type(){
    let f = String::from("hello world");
    let h = &f[0..5];
    let w = &f[6..11];

    println!("{}-{}", h,w);
}