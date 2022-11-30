#[allow(dead_code)]
pub fn compound_type() {
    let f = String::from("hello world".to_string());
    let h = &f[0..5];
    let w = &f[6..11];

    println!("{}-{}", h, w);

    let s = String::from("hello".to_string());
    let word = first(&s);

    // s.clear();

    println!("{}", word);

    let mut str = String::from("hello");
    str.push(',');
    str.push_str("world");

    str.insert(str.len(), '!');
    str.insert_str(str.len(), "!!!");

    str.pop(); //删除并返回字符串的最后一个字符
    str.remove(str.len() - 1); //删除并返回字符串中指定位置的字符
    str.truncate(100); //删除字符串中从指定位置开始到结尾的全部字符
    println!("{}", str.replace("h", "H"));
    println!("{}", str);

    let s1 = String::from("a");
    let s2 = String::from("b");

    println!("{}", s1 + &s2);
}

fn first(s: &String) -> &str {
    &s[..1]
}

#[allow(dead_code)]
pub fn compound_type_practice() {
    let mut s1 = String::new();
    push(&mut s1, "hello");
    println!("{}", s1);
}

fn push(s: &mut String, p: &str) {
    s.push_str(p)
}

#[allow(dead_code)]
pub fn compound_type_tuple() {
    let tup: (i32, i32) = (1, 2);
    println!("{}", tup.1);

    let s = String::from("hello");

    println!("{:?}", tuple_string(s));
}

fn tuple_string(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: i32,
}

fn new_user(s: String, a: i32) -> User {
    User { name: s, age: a }
}

#[allow(dead_code)]
pub fn compound_type_struct() {
    let u = new_user(String::from("a"), 21);
    println!("{:?}", u);

    let u1 = User {
        // name:String::from("123"),
        age: 19,
        ..u
    };

    println!("{:?}", u1);
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs(u8),
    Spades(char),
    Diamonds,
    Hearts,
}

#[allow(dead_code)]
pub fn compound_type_enum() {
    let cl: PokerSuit = PokerSuit::Clubs(9);
    let sp: PokerSuit = PokerSuit::Spades('9');
    println!("{:?}", cl);
    println!("{:?}", sp);
}

#[allow(dead_code)]
pub fn compound_type_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let _array: [String; 8] = core::array::from_fn(|_i| String::from("rust is good!"));
    let name0 = a.get(0).unwrap();
    println!("{:#?}", name0);

    for i in 0..5 {
        println!("{}", i)
    }

    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    let mut n = 0;
    while n < 1 {
        println!("{}", n);
        n += 1
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

#[allow(dead_code)]
pub fn compound_type_match() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let vv = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", vv);

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
