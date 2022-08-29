use crate::common::*;
use crate::provider::*;

pub struct DerivedNode<T, U> {
    read: fn() -> T,
    write: fn(value: U),
}

impl<T, U> DerivedNode<T, U> {
    pub fn new(read: fn() -> T, write: fn(value: U)) -> DerivedNode<T, U> {
        DerivedNode { read, write }
    }
}

impl<T, U> Keyed for DerivedNode<T, U> {}

pub struct DerivedValue<T, U> {
    provided: ProvidedValue<T>,
    dependency_updated: bool,
    node: DerivedNode<T, U>,
}
