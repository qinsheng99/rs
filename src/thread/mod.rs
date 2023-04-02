use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
pub fn test_thread() {
    let h = thread::spawn(|| {
        for i in 1..5 {
            println!("son {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..4 {
        println!("{}", i);
    }

    h.join().unwrap()
}

#[allow(dead_code)]
pub fn test_move() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("{:?}", v);
    });
}

#[allow(dead_code)]
pub fn test_channel() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let a = String::from("abc");
        let _p: () = match sender.send(a) {
            Ok(_t) => (),
            Err(f) => panic!("{}", f),
        };
    });

    let v = receiver.recv().unwrap();

    println!("Got {}", v);
}

#[allow(dead_code)]
pub fn test_channels() {
    let (sender, receiver) = mpsc::channel();

    let k = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        let a = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for i in a {
            k.send(i).unwrap();
            thread::sleep(Duration::from_micros(1))
        }
    });

    for v in receiver {
        println!("Got {}", v);
    }
}
