use std::collections::HashMap;

use crate::common::{NodeKey, Read, Write};
use crate::provider::*;

pub struct RootNode<'a, T: Clone> {
    pub initial_value: T,
    pub key: NodeKey,
    pub values: HashMap<&'a ProviderNode<'a>, ProvidedValue<T>>,
    provider_tree: &'a ProviderTree<'a>,
}

impl<'a, T: Clone> RootNode<'a, T> {
    pub fn new(initial_value: T, provider_tree: &'a ProviderTree) -> RootNode<'a, T> {
        RootNode {
            initial_value,
            key: NodeKey {},
            values: HashMap::new(),
            provider_tree,
        }
    }
}

impl<'a, T: Clone> Read<'a, T> for RootNode<'a, T> {
    fn get(&'a mut self) -> &'a T {
        let provider: &ProviderNode = self.provider_tree.get_current();

        if !self.values.contains_key(provider) {
            let new_value = ProvidedValue::new(self.initial_value.clone());
            self.values.insert(provider, new_value);
        }

        let value: &T = &self.values.get(provider).unwrap().current;

        &value
    }
}

impl<'a, T: Clone> Write<T> for RootNode<'a, T> {
    fn set(&mut self, value: T) {
        let provider: &ProviderNode = self.provider_tree.get_current();

        if !self.values.contains_key(provider) {
            let new_value = ProvidedValue::new(value);
            self.values.insert(provider, new_value);
        } else {
            let mut existing_value = self.values.get_mut(provider).unwrap();
            existing_value.current = value;
        }
    }
}
