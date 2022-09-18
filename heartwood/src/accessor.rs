use std::{fmt::Display, rc::Rc};

use crate::{
    common::{Dependent, Read},
    provider::ProviderTree,
};

pub struct Accessor<'a, T> {
    pub on_change: &'a dyn Fn(Rc<T>),
    source: &'a dyn Accessible<'a, T>,
}

pub trait Accessible<'a, T>: Read<'a, T> + Display {}

impl<'a, T> Accessor<'a, T> {
    pub fn new(source: &'a dyn Accessible<'a, T>, on_change: &'a dyn Fn(Rc<T>)) -> Self {
        Self { on_change, source }
    }

    pub fn current(&self) -> Rc<T> {
        self.source.get()
    }
}

impl<T> Display for Accessor<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Accessor for node {}", self.source)
    }
}

impl<'a, T: Display> Dependent for Accessor<'a, T> {
    fn destroy(&self) {
        let next = self.source.get();
        println!(
            "Accessor for {} detected change, next value: {}",
            self.source, next
        );
        (self.on_change)(next);
    }
}
