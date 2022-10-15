use std::fmt::Debug;
use std::rc::Rc;
use crate::common::{Read, Write};
use crate::v2::accessor::Accessible;
use crate::v2::data_provider::DataProvider;
use crate::v2::provider_tree::{Dependent, Provided, ProviderTree, Scope};

pub struct DerivedNode<T: Debug + 'static, U: 'static> {
    write: &'static dyn Fn(U),
    provider_tree: &'static ProviderTree,
    provider: DataProvider<T>,
    debug_name: &'static str,
}

impl<T: Debug + 'static, U: 'static> DerivedNode<T, U> {
    pub fn new(
        read: &'static dyn Fn() -> T,
        write: &'static dyn Fn(U),
        provider_tree: &'static ProviderTree,
        debug_name: &'static str,
    ) -> Self {
        Self {
            write,
            provider_tree,
            provider: DataProvider::new(read),
            debug_name,
        }
    }

    pub fn read(&'static self) -> Rc<T> {
        {
            self.provider_tree.dependent_stack.stack.borrow_mut().push(self);
        }

        let provider = self.provider_tree.get_current();
        let value = self.provider.get_value(provider.clone());

        let index = self.provider_tree.dependent_stack.stack.borrow().len() - 2;
        println!(
            "{:?} adding dependent: {:?}",
            self,
            self.provider_tree.dependent_stack.stack.borrow().get(index).unwrap()
        );
        self.provider.attach_dependent(
            provider.clone(),
            *self.provider_tree.dependent_stack.stack.borrow().get(index).unwrap(),
        );

        {
            self.provider_tree.dependent_stack.stack.borrow_mut().pop();
        }

        value
    }

    fn write(&self, value: U) {
        println!("set {:?}", self);
        (self.write)(value);
    }
}

impl<T: Debug, U> Read<T> for DerivedNode<T, U> {
    fn get(&self) -> Rc<T> {
        self.read()
    }

    fn getp(&self, scope: &'static Scope) -> Rc<T> {
        let get_value = || { self.read() };

        self.provider_tree.scope_stack.act(scope, &get_value)
    }
}

impl<T: Debug, U> Write<U> for DerivedNode<T, U> {
    fn set(&self, value: U) {
        self.write(value);
    }

    fn setp(&self, value: U, scope: &'static Scope) {
        let write_value = || { self.write(value) };

        self.provider_tree.scope_stack.act(scope, &write_value);
    }
}

impl<T: Debug, U> Dependent for DerivedNode<T, U> {
    fn nudge(&self) {
        let provider = self.provider_tree.get_current();

        self.provider.delete(provider);
    }
}

impl<T: Debug, U> Debug for DerivedNode<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_name)
    }
}

impl<T: Debug, U> Provided for DerivedNode<T, U> {
    fn get_tree(&self) -> &ProviderTree {
        self.provider_tree
    }
}

impl<T: Debug, U> Accessible<T> for DerivedNode<T, U> {}
