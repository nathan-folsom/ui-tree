use std::{fmt::Display, rc::Rc};

use crate::provider::Scope;

pub trait Dependent: Display {
    fn destroy(&self);
}

pub trait Read<'a, T> {
    fn get(&'a self) -> Rc<T>;
    fn getp(&'a self, scope: &'static Scope) -> Rc<T>;
}

pub trait Write<T> {
    fn set(&self, value: T);
    fn setp(&self, value: T, scope: &'static Scope);
}
