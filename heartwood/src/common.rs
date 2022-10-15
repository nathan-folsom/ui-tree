use std::{fmt::Display, rc::Rc};
use crate::v2::provider_tree::Scope;

pub trait Dependent: Display {
    /// This method is called when the value that the implementor is dependent on has changed. It
    /// is up to the implementor to decide how to react to this.
    fn nudge(&self);
}

pub trait Read<T> {
    fn get(&self) -> Rc<T>;
    fn getp(&self, scope: &'static Scope) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
    fn setp(&self, value: T, scope: &'static Scope);
}
