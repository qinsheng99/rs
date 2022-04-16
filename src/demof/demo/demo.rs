pub mod demofs {
    use std::collections::HashMap;

    pub fn demo1() {
        let _tt : (f32,u8,u32) = (3.14,34,56);
        let x :u64 = 16;
        println!("{}",x)
    }

    pub fn demo12() {
        let text = "hello world the world";
        let mut m = HashMap::new();

        for t in text.split_whitespace() {
            let count = m.entry(t).or_insert(0);
            *count += 1;
        }

        println!(
            "{:?}",m
        )
    }
    // #[derive(Debug)]
// struct Color(i32, i32);

// #[derive(Debug)]
// enum IpAddrKind{
//     V4(String),
//     V6(String),
// }

// enum Coin {
//     x,
//     y,
//     z,
// }

// fn demo9(c :Coin) ->u64{
//     match c {
//         Coin::x => 1,
//         Coin::y=> 2,
//         Coin::z=>3,
//     }
// }

// fn demo10(o:Option<i32>) -> Option<i32> {
//     match o {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }

// fn demo11(s:Option<i32>) {
//     if let Some(3) = s {
//         println!("three")
//     }
// }

// struct Rectangle {
//     width : u32,
//     length: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.length
//     }

//     fn squre(size: u32) -> Rectangle {
//         Rectangle{
//             width:size,
//             length:size,
//         }
//     }
// }

// #[derive(Debug)]
// struct User {
//     name: String,
//     email: String,
//     x: u64,
//     y: bool,
// }

// fn give1() ->String {
//     let s = String::from("hello");
//     s
// }

// fn give2(s: String) -> String {
//     s
// }

// fn demo2 (x: u64) -> u64{
//     x+1
// }


// fn demo3(x:u64) {
//     if x < 3 {
//         println!("小于3")
//     } else {
//         println!("大于等于3")
//     }
// }

// fn demo4() {
//     let mut result = 0;
//     let r = loop {
//         result+=1;
//         if result == 10 {
//             break result *2;
//         }
//     };

//     println!("{}", r)
// }

// fn demo5() {
//     let arr:[i64;5]= [10,20,30,40,50];
//     for e in arr.iter() {
//         println!("value is {}", e)
//     }

//     for ee in (1..4).rev() {
//         println!("{}", &ee)
//     }
// }

// fn demo6(s:&String)-> usize {
//     s.len()
// }

//切片引用
// fn demo7(s:&str) ->&str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn demo8() {
//     let uu1 = User {
//         name: String::from("name"),
//         email: String::from("email"),
//         x:16,
//         y:true,
//     };
//     let uu2 = User {
//         name:String::from("name1"),
//         email:String::from("email1"),
//         ..uu1
//     };

//     println!("{}", uu1.name);
//     println!("{}", uu1.email);
//     println!("{}", uu1.x);
//     println!("{}", uu1.y);

//     println!("{}", uu2.name);
//     println!("{}", uu2.email);
//     println!("{}", uu2.x);
//     println!("{}", uu2.y);

// }
}