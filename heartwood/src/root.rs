use std::cell::{Ref, RefCell};
use std::collections::HashMap;

use crate::common::{NodeKey, Read, Write};
use crate::provider::*;

pub struct RootNode<'a, T: Clone> {
    pub key: NodeKey,
    provider: DataProvider<'a, T>,
    pub values: RefCell<HashMap<&'a ProviderNode<'a>, ProvidedValue<T>>>,
    provider_tree: &'a ProviderTree<'a>,
}

impl<'a, T: Clone + 'static> RootNode<'a, T> {
    pub fn new(initial_value: T, provider_tree: &'a ProviderTree) -> RootNode<'a, T> {
        let init_value = || ProvidedValue::new(initial_value.clone());
        RootNode {
            key: NodeKey {},
            values: RefCell::new(HashMap::new()),
            provider_tree,
            provider: DataProvider::new(Box::new(|| ProvidedValue::new(initial_value.clone()))),
        }
    }
}

impl<'a, T: Clone + 'static> Read<'a, T> for RootNode<'a, T> {
    fn get(&'a self) -> std::rc::Rc<T> {
        let provider: &ProviderNode = self.provider_tree.get_current();

        self.provider.get_value(provider)
    }
}

impl<'a, T: Clone> Write<T> for RootNode<'a, T> {
    fn set(&self, value: T) {
        let provider: &ProviderNode = self.provider_tree.get_current();

        if !self.values.borrow().contains_key(provider) {
            let new_value = ProvidedValue::new(value);
            self.values.borrow_mut().insert(provider, new_value);
        } else {
            self.values.borrow_mut().get_mut(provider).unwrap().current = value;
        }
    }
}
