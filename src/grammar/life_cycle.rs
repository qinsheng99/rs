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

#[allow(dead_code)]
fn life<'a: 'b, 'b>(_a: &'a str, b: &'b str) -> &'b str {
    b
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

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(dead_code)]
pub fn life_cycle() {
    let mut x: Vec<i32> = vec![1i32, 2, 3];

    //更新数组
    //push中对数组进行了可变借用,并在push函数退出时销毁这个借用
    x.push(10);

    {
        //可变借用1
        let mut y = &mut x;
        y.push(100);

        //可变借用2,注意：此处是对y的借用,不可再对x进行借用,
        //因为y在此时依然存活。
        let z = &mut y;
        z.push(1000);

        println!("{:?}", z); //打印: [1, 2, 3, 10, 100, 1000]
    } //y和z在此处被销毁,并释放借用。

    //访问x正常
    println!("{:?}", x); //打印: [1, 2, 3, 10, 100, 1000]
}

// pub fn cycle() {
//     let x = 100i64;

//     {
//         let y = &x;
//         println!("{}", y)
//     } //y的作用域结束
// }

//编译器告诉我们,需要我们显式指定Lifetime标识符,因为这个时候,编译器无法推导出返回值的Lifetime应该是比 x长,还是比y长。
//虽然我们在函数中中用了 if true 确认一定可以返回x,但是要知道,编译器是在编译时候检查,而不是运行时,所以编译期间会同时检查所有的输入参数和返回值。
#[allow(dead_code)]
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a:声明一个生命周期
    if x.len() > 2 {
        x
    } else {
        y
    }
}
//
//我们可以显式地告诉编译器'b比'a长（'a是'b的子集）,只需要在定义Lifetime的时候, 在'b的后面加上: 'a, 意思是'b比'a长,'a是'b的子集:
#[allow(dead_code)]
fn foo1<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > 2 {
        x
    } else {
        y
    }
}
// //条件：Lifetime(x) ⊆ Lifetime(y)
//     推导：Lifetime(返回值) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

//     即：

//     条件： 'a ⊆ 'b
//     推导：'a ⊆ ('a ∩ 'b) // 成立
