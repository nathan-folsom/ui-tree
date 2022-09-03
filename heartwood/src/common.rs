use std::rc::Rc;

pub trait Dependent {}

pub trait Read<'a, T> {
    fn get(&'a self) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
}
