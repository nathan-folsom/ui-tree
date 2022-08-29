use std::any::Any;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct NodeKey;

pub trait Keyed {
    const KEY: NodeKey = NodeKey {};
}

pub trait Dependent {}

pub trait Read<'a, T> {
    fn get(&'a mut self) -> &'a T;
}

pub trait Write<T> {
    fn set(&mut self, value: T);
}
