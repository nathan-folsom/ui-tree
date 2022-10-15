use std::fmt::Debug;
use std::rc::Rc;
use crate::common::Read;
use crate::v2::provider_tree::{Dependent, Provided};

pub struct Accessor<T> {
    pub on_change: &'static dyn Fn(Rc<T>),
    source: &'static dyn AccessorSource<T>,
}

trait AccessorSource<T>: Accessible<T> + Debug {}

pub trait Accessible<T>: Read<T> + Provided {}

impl<T> Accessor<T> {
    pub fn new(source: &'static dyn Accessible<T>, on_change: &'static dyn Fn(Rc<T>)) -> Self {
        Self { on_change, source }
    }

    pub fn current(&self) -> Rc<T> {
        {
            self.source.get_tree().call_stack.borrow_mut().push(self)
        }

        let val = self.source.get();

        {
            self.source.get_tree().call_stack.borrow_mut().pop();
        }

        val
    }
}

impl<T> Debug for Accessor<T> {
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
