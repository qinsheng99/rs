macro_rules! log {
    ($($x:expr), *) => {
       $(
            println!("{:?}", $x);
       )*
    };
}

macro_rules! map {
    ($($k:expr => $v:expr), *) => {{
        let mut hmp = ::std::collections::HashMap::new();
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
