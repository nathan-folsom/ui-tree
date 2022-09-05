use crate::common::Dependent;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

pub const GLOBAL_SCOPE: Scope = Scope();

pub struct ProviderTree<'a> {
    root: ProviderNode<'a>,
    pub call_stack: RefCell<Vec<&'a dyn Dependent>>,
}

impl<'a> ProviderTree<'a> {
    /// Supplying an initial_dependent allows reading node values at arbitrary places in your code.
    /// This can be helpful for testing, but the value will not be reactive and could become
    /// stale if values are reset and .get() is not called again on the relevant node.
    pub fn new(initial_dependent: Option<&'a dyn Dependent>) -> ProviderTree<'a> {
        let call_stack = if let Some(i) = initial_dependent {
            vec![i]
        } else {
            vec![]
        };

        ProviderTree {
            root: ProviderNode {
                scope: GLOBAL_SCOPE,
                parent: None,
            },
            call_stack: RefCell::new(call_stack),
        }
    }

    pub fn get_current(&self) -> &ProviderNode {
        &self.root
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ProviderNode<'a> {
    scope: Scope,
    parent: Option<Box<&'a ProviderNode<'a>>>,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Scope();

pub struct ProvidedValue<'a, T> {
    pub current: Rc<T>,
    pub dependents: RefCell<Vec<&'a dyn Dependent>>,
}

impl<'a, T> ProvidedValue<'a, T> {
    pub fn new(initial_value: T) -> ProvidedValue<'a, T> {
        ProvidedValue {
            current: Rc::new(initial_value),
            dependents: RefCell::new(Vec::new()),
        }
    }
}

pub struct DataProvider<'a, T: Display> {
    values: ValueProvider<'a, T>,
    init_value: &'a dyn Fn() -> T,
}

type ValueProvider<'a, T> = RefCell<HashMap<&'a ProviderNode<'a>, ProvidedValue<'a, T>>>;

impl<'a, T: Display> DataProvider<'a, T> {
    pub fn new(init_value: &'a dyn Fn() -> T) -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            init_value,
        }
    }
    pub fn get_value(&self, provider: &'a ProviderNode) -> Rc<T> {
        let initialized = { self.values.borrow().contains_key(provider) };

        if initialized {
            return Rc::clone(&self.values.borrow().get(provider).unwrap().current);
        }

        {
            let new_value = ProvidedValue::new((self.init_value)());
            println!("Initializing value: {}", new_value.current);
            let mut values = self.values.borrow_mut();
            values.insert(provider, new_value);
        }
        return Rc::clone(&self.values.borrow().get(provider).unwrap().current);
    }
    pub fn set_value(&self, provider: &'a ProviderNode, value: T) {
        let initialized = { self.values.borrow().contains_key(provider) };

        if initialized {
            self.values.borrow_mut().get_mut(provider).unwrap().current = Rc::new(value);
        } else {
            let new_value = ProvidedValue::new(value);
            self.values.borrow_mut().insert(provider, new_value);
        }
    }

    pub fn delete(&self, provider: &'a ProviderNode) {
        self.notify_dependents(provider);
        self.values.borrow_mut().remove_entry(provider);
    }

    pub fn attach_dependent(&self, provider: &'a ProviderNode, dependent: &'a dyn Dependent) {
        self.values
            .borrow()
            .get(provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .push(dependent);
    }

    pub fn notify_dependents(&self, provider: &'a ProviderNode) {
        self.values
            .borrow()
            .get(provider)
            .unwrap()
            .dependents
            .borrow()
            .iter()
            .for_each(|d| d.destroy());
        self.values
            .borrow()
            .get(provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .drain(0..);
    }
}
