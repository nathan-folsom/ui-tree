use crate::common::{Read, Write};
use crate::provider::*;

pub struct RootNode<'a, T: Clone> {
    provider: DataProvider<'a, T>,
    provider_tree: &'a ProviderTree<'a>,
}

impl<'a, T: Clone + 'static> RootNode<'a, T> {
    pub fn new(init: &'a dyn Fn() -> T, provider_tree: &'a ProviderTree) -> RootNode<'a, T> {
        RootNode {
            provider_tree,
            provider: DataProvider::new(init),
        }
    }
}

impl<'a, T: Clone + 'a> Read<'a, T> for RootNode<'a, T> {
    fn get(&'a self) -> std::rc::Rc<T> {
        let provider: &ProviderNode = self.provider_tree.get_current();

        self.provider.get_value(provider)
    }
}

impl<'a, T: Clone> Write<T> for RootNode<'a, T> {
    fn set(&self, value: T) {
        let provider: &ProviderNode = self.provider_tree.get_current();

        self.provider.set_value(provider, value);
    }
}
