#[allow(dead_code)]
pub fn list() {
    let mut v1:Vec<usize> = vec![];
    push(&mut v1, 50_000_000);

    let mut v2:Vec<usize> = vec![];
    v2.reserve(50_000_000);
    push(&mut v2, 50_000_000)
}
use std::time::SystemTime;
fn push(v:&mut Vec<usize>, total: usize) {
    let e1 = SystemTime::now();

    for i in 1..total {
        v.push(i)
    }

    let e2 = SystemTime::now();

    println!("cost time {:?}", e2.duration_since(e1).unwrap())
}

use std::collections::hash_map::HashMap;
#[allow(dead_code)]
pub fn map() {
    // 声明
    let mut come_from = HashMap::new();
// 插入
    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S.");
    come_from.insert("Mike", "HuoGuo");

// 查找key
    if !come_from.contains_key("elton") {
        println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
    }

// 根据key删除元素
    come_from.remove("Mike");
    println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

// 利用get的返回判断元素是否存在
    let who = ["MoGu", "Marisa"];
    for person in &who {
        match come_from.get(person) {
            Some(location) => println!("{} 来自: {}", person, location),
            None => println!("{} 也无家可归啊.", person),
        }
    }

// 遍历输出
    println!("那么，所有人呢？");
    for (name, location) in &come_from {
        println!("{}---来自: {}", name, location);
    }
}
#[allow(dead_code)]
pub fn map1() {
    let mut l = HashMap::new();

    for i in "a short treatise on fungi".chars() {
        if i == ' ' {
            continue
        }
        let c = l.entry(i).or_insert(0);
        *c += 1;
    }

    println!("{:?}", l)
}
#[allow(dead_code)]
pub fn iter(){
    IntoIterator::into_iter()
}
