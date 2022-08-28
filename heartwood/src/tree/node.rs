pub trait Read<T> {
    fn read(&self) -> T;
}

pub trait Write<T> {
    fn write(&self, value: T);
}

pub struct Node<'a, R> {
    subscribers: Vec<Box<&'a dyn Write<R>>>,
    value: R,
    updated: bool
}

impl<'a, R> Node<'a, R> {
    pub fn sub(mut self, subscriber: &'a dyn Write<R>) {
        let s = Box::new(subscriber);
        self.subscribers.push(s);
    }
}
