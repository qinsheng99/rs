use std::{mem::size_of_val,collections::hash_map::HashMap};
use num::complex::{Complex, Complex64};

#[allow(dead_code)]
pub fn form_demo(){
    let guss = "42".as_bytes();

    println!("{:?}", guss);

    let a:u8 = 255;
    let b = a.wrapping_add(19);
    println!("{}", b);


    let abc:(f32,f32,f32) = (0.1,0.2,0.3);
    println!("{}", abc.0 + abc.1);
    println!("{}", abc.2);

    let xyz:(f64,f64,f64) = (0.1,0.2,0.3);
    println!("{}", xyz.0 + xyz.1);
    println!("{}", xyz.2);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='c'{
        println!("{}", i);
    }

    let com1 = Complex::new(2.1,-1.2);
    let com2 = Complex64::new(11.1, 22.2);
    let result = com1 + com2;

    println!("{} + {}i", result.re, result.im);

    let zh = '中';
    println!("{}", size_of_val(&zh));

    let mut m:HashMap<&str,()> = HashMap::new();

    m.insert("k", ());

    println!("{:?}", m);

    assert_eq!(3,sum(1,2));

    let mut s = String::new();

    str(&mut s);

    println!("{}", s);

    let numx = 5;
    let numy = &numx;

    println!("{}", numx);
    println!("{}", *numy);


}

fn sum(x:i32,y:i32) ->i32 {
    //return x+y;
    x + y
}

fn str(x :&mut String)->() {
    *x = String::from("ceshi")
}