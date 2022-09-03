use std::rc::Rc;

use crate::common::*;
use crate::provider::*;

pub struct DerivedNode<'a, T, U> {
    write: &'a dyn Fn(U),
    provider_tree: &'a ProviderTree<'a>,
    provider: DataProvider<'a, T>,
}

impl<'a, T, U> DerivedNode<'a, T, U> {
    pub fn new(
        read: &'a dyn Fn() -> T,
        write: &'a dyn Fn(U),
        provider_tree: &'a ProviderTree,
    ) -> Self {
        Self {
            write,
            provider_tree,
            provider: DataProvider::new(read),
        }
    }
}

impl<'a, T, U> Read<'a, T> for DerivedNode<'a, T, U> {
    fn get(&'a self) -> Rc<T> {
        let provider = self.provider_tree.get_current();

        self.provider.get_value(&provider)
    }
}

impl<'a, T, U> Write<U> for DerivedNode<'a, T, U> {
    fn set(&self, value: U) {
        (self.write)(value);
    }
}
