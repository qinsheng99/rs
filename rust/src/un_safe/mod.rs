#[allow(dead_code)]
pub fn demo() {
    let mut num = 6;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 9;

        println!("{}", *r1);
        println!("{}", *r2);
    }
}

#[allow(dead_code)]
pub fn unsafe_split() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    println!("{:?}", split_at_mut(r, 3));
}

#[allow(dead_code)]
fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let l = v.len();
    let p = v.as_mut_ptr();

    assert!(mid <= l);

    unsafe {
        (
            std::slice::from_raw_parts_mut(p, mid),
            std::slice::from_raw_parts_mut(p.add(mid), l - mid),
        )
    }
}

#[allow(dead_code)]
extern "C" {
    fn abc(i: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_c() {
    println!("{}", "call c");
}
