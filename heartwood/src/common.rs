use std::{fmt::Display, rc::Rc};

use crate::provider::Scope;

pub trait Dependent: Display {
    /// This method is called when the value that the implementor is dependent on has changed. It
    /// is up to the implementor to decide how to react to this.
    fn nudge(&self);
}

pub trait Read<'a, T> {
    fn get(&'a self) -> Rc<T>;
    fn getp(&'a self, scope: &'static Scope) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
    fn setp(&self, value: T, scope: &'static Scope);
}
