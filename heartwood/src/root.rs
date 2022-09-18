use std::fmt::Display;
use std::rc::Rc;

use crate::accessor::Accessible;
use crate::common::{Read, Write};
use crate::provider::*;

pub struct RootNode<'a, T: Clone + Display> {
    provider: DataProvider<'a, T>,
    provider_tree: &'a ProviderTree<'a>,
    debug_name: &'static str,
}

impl<'a, T: Clone + Display> RootNode<'a, T> {
    pub fn new(
        init: &'a dyn Fn() -> T,
        provider_tree: &'a ProviderTree<'a>,
        debug_name: &'static str,
    ) -> Self {
        Self {
            provider_tree,
            provider: DataProvider::new(init),
            debug_name,
        }
    }

    fn read(&'a self) -> Rc<T> {
        let provider = self.provider_tree.get_current();
        let value = self.provider.get_value(provider.clone());

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

    fn write(&self, value: T) {
        let provider = self.provider_tree.get_current();

        println!("set root");
        self.provider.set_value(provider.clone(), value);

        self.provider.notify_dependents(provider.clone());
    }
}

impl<'a, T: Clone + Display + 'static> Read<'a, T> for RootNode<'a, T> {
    fn getp(&'a self, scope: &'static Scope) -> Rc<T> {
        {
            self.provider_tree.scope_stack.borrow_mut().push(scope);
        }

        let value = self.read();

        {
            self.provider_tree.scope_stack.borrow_mut().pop();
        }

        value
    }

    fn get(&'a self) -> Rc<T> {
        self.read()
    }
}

impl<'a, T: Clone + Display> Write<T> for RootNode<'a, T> {
    fn set(&self, value: T) {
        self.write(value);
    }

    fn setp(&self, value: T, scope: &'static Scope) {
        {
            self.provider_tree.scope_stack.borrow_mut().push(scope);
        }

        self.write(value);

        {
            self.provider_tree.scope_stack.borrow_mut().pop();
        }
    }
}

impl<T: Display + Clone> Display for RootNode<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_name)
    }
}

impl<'a, T: Clone + Display + 'static> Accessible<'a, T> for RootNode<'a, T> {}
