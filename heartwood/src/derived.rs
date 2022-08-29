use std::collections::HashMap;
use std::marker::PhantomData;

use crate::common::*;
use crate::provider::*;

pub struct DerivedNode<'a, T, U, R: FnMut() -> T, W: FnMut(U)> {
    read: R,
    write: W,
    pub key: NodeKey,
    pub values: HashMap<&'a ProviderNode<'a>, ProvidedValue<T>>,
    provider_tree: &'a ProviderTree<'a>,
    _t: PhantomData<(T, U)>,
}

impl<T, U, R: FnMut() -> T, W: FnMut(U)> DerivedNode<'_, T, U, R, W> {
    pub fn new(
        read: R,
        write: W,
        provider_tree: &'static ProviderTree,
    ) -> DerivedNode<'static, T, U, R, W> {
        DerivedNode {
            read,
            write,
            key: NodeKey {},
            provider_tree,
            values: HashMap::new(),
            _t: Default::default(),
        }
    }
}

impl<T, U, R: FnMut() -> T, W: FnMut(U)> Keyed for DerivedNode<'_, T, U, R, W> {}

impl<'a, T, U, R: FnMut() -> T, W: FnMut(U)> Read<'a, T> for DerivedNode<'a, T, U, R, W> {
    fn get(&'a mut self) -> &'a T {
        let provider = self.provider_tree.get_current();

        if !self.values.contains_key(provider) {
            let new_value = ProvidedValue::new((self.read)());
            self.values.insert(provider, new_value);
        }

        let current_value: &T = &self.values.get_mut(provider).unwrap().current;

        current_value
    }
}