pub mod english;
mod japanese {
    mod greetings {
    }

    mod farewells {
    }
}

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

pub fn forth(){
    crate::first::second::third();
}

use crate::first::second;

pub fn seven() {
    second::third();
}