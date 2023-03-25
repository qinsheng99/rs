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
}
