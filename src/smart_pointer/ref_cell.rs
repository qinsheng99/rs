use std::cell::RefCell;

pub trait Message {
    fn send(&self, name: &str);
}

#[allow(dead_code)]
struct Danger<'a, T: 'a + Message> {
    m: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> Danger<'a, T>
where
    T: Message,
{
    #[allow(dead_code)]
    fn new(m: &T, max: usize) -> Danger<T> {
        Danger { m, value: 0, max }
    }

    #[allow(dead_code)]
    fn set_value(&mut self, value: usize) {
        self.value = value;

        let v = self.value as f32 / self.max as f32;

        if v > 1.0 {
            self.m
                .send(&*format!("the value: {} is dangerous", self.value));
        }

        self.m.send(&*format!("set value: {}", self.value))
    }
}

#[allow(dead_code)]
struct Messages {
    data: RefCell<Vec<String>>,
}

impl Messages {
    fn new() -> Messages {
        Messages {
            data: RefCell::new(vec![]),
        }
    }
}

impl Message for Messages {
    fn send(&self, name: &str) {
        self.data.borrow_mut().push(String::from(name))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let me = Messages::new();

        let mut d = Danger::new(&me, 100);

        d.set_value(90);

        assert_eq!(me.data.borrow().len(), 2);
    }
}
