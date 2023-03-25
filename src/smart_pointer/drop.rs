#[allow(dead_code)]
struct DropSmartPoint {
    data: String,
}

impl Drop for DropSmartPoint {
    fn drop(&mut self) {
        println!("{}", self.data)
    }
}

#[allow(dead_code)]
pub fn dbox() {
    let _a = DropSmartPoint {
        data: String::from("123"),
    };

    drop(_a);
    let _b = DropSmartPoint {
        data: String::from("456"),
    };

    println!("create")
}
