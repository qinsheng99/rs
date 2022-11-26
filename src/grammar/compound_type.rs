#[allow(dead_code)]
pub fn compound_type(){
    let f = String::from("hello world".to_string());
    let h = &f[0..5];
    let w = &f[6..11];

    println!("{}-{}", h,w);

    let s = String::from("hello".to_string());
    let word = first(&s);

    // s.clear();

    println!("{}", word);

    let mut str = String::from("hello");
    str.push(',');
    str.push_str("world");

    str.insert(str.len(), '!');
    str.insert_str(str.len(), "!!!");

    str.pop();  //删除并返回字符串的最后一个字符
    str.remove(str.len() - 1);    //删除并返回字符串中指定位置的字符
    str.truncate(100); //删除字符串中从指定位置开始到结尾的全部字符
    println!("{}",str.replace("h", "H"));
    println!("{}", str);

    let s1 = String::from("a");
    let s2 = String::from("b");

    println!("{}",s1 + &s2);

}

fn first(s: &String) -> &str{
    &s[..1]
}

#[allow(dead_code)]
pub fn compound_type_practice(){
    let mut s1 = String::new();
    push(&mut s1, "hello");
    println!("{}", s1);
}

fn push(s: &mut String, p: &str){
    s.push_str(p)
}