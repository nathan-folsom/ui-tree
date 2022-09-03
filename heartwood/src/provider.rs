use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct ProviderTree<'a> {
    root: ProviderNode<'a>,
}

impl ProviderTree<'static> {
    pub fn new() -> ProviderTree<'static> {
        ProviderTree {
            root: ProviderNode {
                scope: None,
                parent: None,
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ProviderNode<'a> {
    scope: Option<Scope>,
    parent: Option<Box<&'a ProviderNode<'a>>>,
}

impl ProviderTree<'_> {
    pub fn get_current(&self) -> &ProviderNode {
        &self.root
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct Scope();

pub struct ProvidedValue<T> {
    pub current: Rc<T>,
    pub dependents: Vec<Box<dyn Dependent>>,
}

impl<T> ProvidedValue<T> {
    pub fn new(initial_value: T) -> ProvidedValue<T> {
        ProvidedValue {
            current: Rc::new(initial_value),
            dependents: Vec::new(),
        }
    }
}

pub trait Dependent {}

pub struct DataProvider<'a, T> {
    values: ValueProvider<'a, T>,
    init_value: Box<dyn Fn() -> ProvidedValue<T>>,
}

type ValueProvider<'a, T> = RefCell<HashMap<&'a ProviderNode<'a>, ProvidedValue<T>>>;

impl<'a, T> DataProvider<'a, T> {
    pub fn new(init_value: Box<dyn Fn() -> ProvidedValue<T>>) -> DataProvider<'a, T> {
        DataProvider {
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
            let new_value = (self.init_value)();
            let mut values = self.values.borrow_mut();
            values.insert(provider, new_value);
        }
        return Rc::clone(&self.values.borrow().get(provider).unwrap().current);
    }
}
