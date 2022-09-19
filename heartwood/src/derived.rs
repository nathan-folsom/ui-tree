use std::fmt::Display;
use std::rc::Rc;

use crate::common::*;
use crate::provider::*;

pub struct DerivedNode<'a, T: Display, U> {
    write: &'a dyn Fn(U),
    provider_tree: &'a ProviderTree<'a>,
    provider: DataProvider<'a, T>,
    debug_name: String,
}

impl<'a, T: Display, U> DerivedNode<'a, T, U> {
    pub fn new(
        read: &'a dyn Fn() -> T,
        write: &'a dyn Fn(U),
        provider_tree: &'a ProviderTree<'a>,
        debug_name: &str,
    ) -> Self {
        Self {
            write,
            provider_tree,
            provider: DataProvider::new(read),
            debug_name: String::from(debug_name),
        }
    }

    pub fn read(&'a self) -> Rc<T> {
        {
            self.provider_tree.call_stack.borrow_mut().push(self);
        }

        let provider = self.provider_tree.get_current();
        let value = self.provider.get_value(provider.clone());

        let index = self.provider_tree.call_stack.borrow().len() - 2;
        println!(
            "{} adding dependent: {}",
            self,
            self.provider_tree.call_stack.borrow().get(index).unwrap()
        );
        self.provider.attach_dependent(
            provider.clone(),
            *self.provider_tree.call_stack.borrow().get(index).unwrap(),
        );

        {
            self.provider_tree.call_stack.borrow_mut().pop();
        }

        value
    }

    fn write(&self, value: U) {
        println!("set {}", self);
        (self.write)(value);
    }
}

impl<'a, T: Display, U> Read<'a, T> for DerivedNode<'a, T, U> {
    fn get(&'a self) -> Rc<T> {
        self.read()
    }

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
}

impl<'a, T: Display, U> Write<U> for DerivedNode<'a, T, U> {
    fn set(&self, value: U) {
        self.write(value);
    }

    fn setp(&self, value: U, scope: &'static Scope) {
        {
            self.provider_tree.scope_stack.borrow_mut().push(scope);
        }

        self.write(value);

        {
            self.provider_tree.scope_stack.borrow_mut().pop();
        }
    }
}

impl<'a, T: Display, U> Dependent for DerivedNode<'a, T, U> {
    fn destroy(&self) {
        let provider = self.provider_tree.get_current();

        self.provider.delete(provider);
    }
}

impl<'a, T: Display, U> Display for DerivedNode<'a, T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_name)
    }
}

impl<'a, T: Display, U> Provided<'a> for DerivedNode<'a, T, U> {
    fn get_tree(&'a self) -> &'a ProviderTree {
        self.provider_tree
    }
}
