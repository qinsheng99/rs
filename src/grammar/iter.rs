#[derive(Debug)]
#[allow(dead_code)]
pub struct Shoe {
    size: u32,
    typ: String,
}

#[allow(dead_code)]
pub fn filter(s: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    s.into_iter().filter(|x| x.size == size).collect()
}

#[allow(dead_code)]
pub fn shoes() {
    let mut s: Vec<Shoe> = vec![];
    s.push(Shoe {
        size: 1,
        typ: String::from("lining"),
    });

    s.push(Shoe {
        size: 2,
        typ: String::from("kuangwei"),
    });

    for i in filter(s, 1) {
        println!("{}-{}", i.size, i.typ);
    }
}

#[allow(dead_code)]
struct Count {
    count: u32,
}

impl Count {
    #[allow(dead_code)]
    fn new() -> Count {
        Count { count: 0 }
    }
}

impl Iterator for Count {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            {
                None
            }
        }
    }
}

#[allow(dead_code)]
pub fn iter() {
    let mut c = Count::new();
    println!("{}", c.next().unwrap());

    let sum: u32 = Count::new()
        .zip(Count::new().skip(1))
        .map(|(a, b)| a * b)
        // .filter(|x| x % 3 == 0)
        .sum();
    // 1 2 3 4 5
    // 2 3 4 5
    // 2 6 12 20

    println!("{}", sum)
}
