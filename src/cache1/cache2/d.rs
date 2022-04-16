pub fn dd() {
    println!("dd")
}
pub fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    fn dec(n: i32) -> i32 {
        n - 1
    }
    if n % 2 == 0 {
        inc
    } else {
        dec
    }
}

pub fn highfun() {
    let a = [1,2,3,4,5,6,7];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i));
    }
    println!("{:?}", b);
}

pub fn mth() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_)}) => println!("{:?}", a),
        _ => {}
    }
}


    pub fn f(n:i32) ->fn(i32) ->i32 {
        fn inc(n: i32) -> i32 {
            n + 1
        }
        fn dec(n: i32) -> i32 {
            n - 1
        }
        if n % 2 == 0 {
            inc
        } else {
            dec
        }
    }


pub fn inc(n: i32) -> i32 {//函数定义
    n + 1
}

pub type IncType = fn(i32) -> i32;//函数类型