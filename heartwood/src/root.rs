use std::fmt::Display;

use crate::common::{Read, Write};
use crate::provider::*;

pub struct RootNode<'a, T: Clone + Display> {
    provider: DataProvider<'a, T>,
    provider_tree: &'a ProviderTree<'a>,
}

impl<'a, T: Clone + Display + 'static> RootNode<'a, T> {
    pub fn new(init: &'a dyn Fn() -> T, provider_tree: &'a ProviderTree<'a>) -> Self {
        Self {
            provider_tree,
            provider: DataProvider::new(init),
        }
    }
}

impl<'a, T: Clone + Display + 'a> Read<'a, T> for RootNode<'a, T> {
    fn get(&'a self) -> std::rc::Rc<T> {
        let provider: &ProviderNode = self.provider_tree.get_current();

        let value = self.provider.get_value(provider);

        println!(
            "Adding dependent to root: {}",
            self.provider_tree.call_stack.borrow().last().unwrap()
        );
        self.provider.attach_dependent(
            provider,
            *self.provider_tree.call_stack.borrow().last().unwrap(),
        );

        value
    }
}

impl<'a, T: Clone + Display> Write<T> for RootNode<'a, T> {
    fn set(&self, value: T) {
        let provider: &ProviderNode = self.provider_tree.get_current();

        println!("set root");
        self.provider.set_value(provider, value);

        self.provider.notify_dependents(provider);
    }
}
