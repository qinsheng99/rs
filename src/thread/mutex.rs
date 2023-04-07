#[allow(dead_code)]
pub fn metux() {
    let m = std::sync::Mutex::new(5);

    {
        let mut mm = m.lock().unwrap();
        *mm = 6
    }

    println!("{:?}", m);
}

#[allow(dead_code)]
pub fn threds_metux() {
    use std::sync::{Arc, Mutex};
    let c = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    #[allow(unused)]
    for _i in 0..10 {
        let cc = Arc::clone(&c);

        let h = std::thread::spawn(move || {
            let mut mm = cc.lock().unwrap();

            *mm += 1
        });

        handles.push(h)
    }

    for h in handles {
        h.join().unwrap()
    }

    println!("{:?}", *c.lock().unwrap());
}
