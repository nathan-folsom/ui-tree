use std::fmt::Debug;
use std::rc::Rc;
use crate::common::Read;
use crate::v2::provider_tree::{Dependent, Provided};

pub struct Accessor<T: Debug + 'static> {
    pub on_change: &'static dyn Fn(Rc<T>),
    source: &'static dyn AccessorSource<T>,
}

trait AccessorSource<T>: Accessible<T> + Debug {}

pub trait Accessible<T>: Read<T> + Provided {}

impl<T: Debug> Accessor<T> {
    pub fn new(source: &'static dyn AccessorSource<T>, on_change: &'static dyn Fn(Rc<T>)) -> Self {
        Self { on_change, source }
    }

    pub fn current(&'static self) -> Rc<T> {
        let get_val = || { self.source.get() };

        self.source.get_tree().dependent_stack.act(self, &get_val)
    }
}

impl<T: Debug> Debug for Accessor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Accessor for node {:?}", self.source)
    }
}

impl<T: Debug> Dependent for Accessor<T> {
    fn nudge(&self) {
        let next = self.source.get();
        println!(
            "Accessor for {:?} detected change, next value: {:?}",
            self.source, next
        );
        (self.on_change)(next);
    }
}
