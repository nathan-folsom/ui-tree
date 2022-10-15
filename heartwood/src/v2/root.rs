use std::fmt::Debug;
use std::rc::Rc;
use crate::common::{Read, Write};
use crate::v2::accessor::Accessible;
use crate::v2::data_provider::DataProvider;
use crate::v2::provider_tree::{Provided, ProviderTree, Scope};

pub struct RootNode<T: Debug + 'static> {
    provider: DataProvider<T>,
    provider_tree: &'static ProviderTree,
    debug_name: &'static str,
}

impl<T: Debug + 'static> RootNode<T> {
    pub const fn new(init: &'static dyn Fn() -> T, provider_tree: &'static ProviderTree, debug_name: &'static str) -> Self {
        Self {
            provider_tree,
            provider: DataProvider::new(init),
            debug_name,
        }
    }

    fn read(&self) -> Rc<T> {
        let provider = self.provider_tree.get_current();
        let value = self.provider.get_value(provider.clone());

        println!(
            "Adding dependent to root: {:?}",
            self.provider_tree.dependent_stack.stack.borrow().last().unwrap()
        );
        self.provider.attach_dependent(
            provider,
            *self.provider_tree.dependent_stack.stack.borrow().last().unwrap(),
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

impl<T: Debug + 'static> Read<T> for RootNode<T> {
    fn getp(&self, scope: &'static Scope) -> Rc<T> {
        let get_value = || { self.read() };

        self.provider_tree.scope_stack.act(scope, &get_value)
    }

    fn get(&self) -> Rc<T> {
        self.read()
    }
}

impl<T: Debug> Write<T> for RootNode<T> {
    fn set(&self, value: T) {
        self.write(value);
    }

    fn setp(&self, value: T, scope: &'static Scope) {
        let write_value= || { self.write(value) };

        self.provider_tree.scope_stack.act(scope, &write_value);
    }
}

impl<T: Debug> Debug for RootNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_name)
    }
}

impl<T: Debug + 'static> Accessible<T> for RootNode<T> {}

impl<T: Debug> Provided for RootNode<T> {
    fn get_tree(&self) -> &'static ProviderTree {
        self.provider_tree
    }
}
