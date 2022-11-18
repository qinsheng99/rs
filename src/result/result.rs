use std::fs::File;
use std::io::ErrorKind;
#[allow(dead_code)]
pub fn open1(){
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("creat file err {:?}", e)
                }
            },
            _other=> {
                panic!("open file err {:?}", error)
            }

        }
    };
}

#[allow(dead_code)]
pub fn open2(){
    let _f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("creat file err {:?}", error)
            })
        } else {
            panic!("open file err {:?}", error)
        }
    });


    let _ff  = File::open("").unwrap();
}