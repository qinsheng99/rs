mod japanese {
    mod greetings {
    }

    mod farewells {
    }
}
#[allow(dead_code)]
fn five(){}
pub mod first {
    pub mod second {
        pub fn third() {
            println!("third")
        }
    }

    pub fn six() {
        super::five();
    }
}
#[allow(dead_code)]
pub fn forth(){
    crate::first::second::third();
}

use crate::first::second;
#[allow(dead_code)]
pub fn seven() {
    second::third();
}