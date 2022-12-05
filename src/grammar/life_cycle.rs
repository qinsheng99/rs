#[allow(dead_code)]
pub fn max_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    life_struct_field();
    // 生命周期取参数中短的那个
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct A<'a> {
    path: &'a String,
}

#[allow(dead_code)]
fn life_struct_field() {
    let a = String::from("123");

    let b = A { path: &a };
    println!("{:?}", b);
}
