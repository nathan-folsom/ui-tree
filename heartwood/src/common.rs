use std::rc::Rc;

pub trait Dependent {
    fn destroy(&self);
}

pub trait Read<'a, T> {
    fn get(&'a self) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
}
