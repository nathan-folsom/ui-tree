use std::rc::Rc;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct NodeKey;

pub trait Keyed {
    const KEY: NodeKey = NodeKey {};
}

pub trait Dependent {}

pub trait Read<'a, T> {
    fn get(&'a self) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
}
