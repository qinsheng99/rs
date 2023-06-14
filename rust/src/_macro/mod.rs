use std::collections::HashMap;

macro_rules! log {
    ($($x:expr), *) => {
       $(
            println!("{:?}", $x);
       )*
    };
}

macro_rules! map {
    ($($k:expr => $v:expr), *) => {{
        let mut hmp = HashMap::new();
        $(
            hmp.insert($k,$v);
        )*

        hmp
    }};
}

pub fn v() {
    let v = map!("name"=>"v");

    log!(v);
}
