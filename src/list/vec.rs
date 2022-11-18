#[allow(dead_code)]
pub fn v(){
    let  v1:Vec<i64> =  vec![1,2,3,4,5,6];

    let v2 = v1.iter().cloned().take(2).collect::<Vec<i64>>();
    assert_eq!(v2,vec![1,2]);

    let v3:Vec<i64> = v1.iter().cloned().skip(2).collect();
    assert_eq!(v3,vec![3,4,5,6])
}

use std::collections::hash_map::HashMap;
#[allow(dead_code)]
pub fn z(){
    let v1 = vec!["a", "b", "c"];
    let v2 = vec![1,2,3];

    let v3:HashMap<_,_> = v1.iter().zip(v2.iter()).collect();

    println!("{:?}", v3)
}
#[allow(dead_code)]
pub fn e(){
    let v = vec![1, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 迭代生成标，并且每两个元素剔除一个
        .filter(|&(idx, _)| idx % 2 == 0)
        // 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用
        // .unzip::<_,_, vec<_>, vec<_>>().1
        .map(|(_idx, val)| val)
        // 累加 1+3+5 = 9
        .fold(0, |sum, acm| sum + acm);

    println!("{}", val);
}