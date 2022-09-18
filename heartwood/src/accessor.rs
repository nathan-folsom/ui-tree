use std::rc::Rc;

use crate::common::Read;

pub struct Accessor<'a, T> {
    pub current: Rc<T>,
    pub on_change: &'a dyn FnMut(T),
}

impl<'a, T> Accessor<'a, T> {
    pub fn new(source: &'a dyn Read<'a, T>, on_change: &'a dyn FnMut(T)) -> Self {
        Self {
            current: source.get().clone(),
            on_change,
        }
    }
}
