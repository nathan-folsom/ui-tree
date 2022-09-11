use crate::common::Dependent;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

pub const GLOBAL_SCOPE: Scope = Scope();

pub struct ProviderTree<'a> {
    pub root: Rc<ProviderNode>,
    pub call_stack: RefCell<Vec<&'a dyn Dependent>>,
    pub scope_stack: RefCell<Vec<&'a Scope>>,
    current: RefCell<Option<Rc<ProviderNode>>>,
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

        let root = Rc::new(ProviderNode {
            scope: GLOBAL_SCOPE,
            parent: None,
        });

        ProviderTree {
            root: root.clone(),
            call_stack: RefCell::new(call_stack),
            scope_stack: RefCell::new(vec![&GLOBAL_SCOPE]),
            current: RefCell::new(None),
        }
    }

    fn get_base(&self) -> Rc<ProviderNode> {
        if let Some(p) = &*self.current.borrow() {
            p.clone()
        } else {
            self.root.clone()
        }
    }

    pub fn get_current(&self) -> Rc<ProviderNode> {
        let mut current = self.get_base();
        let scope = &**self.scope_stack.borrow().last().unwrap();

        loop {
            if current.scope == *scope {
                break;
            }

            let parent = &current.parent;

            if let Some(p) = parent {
                current = p.clone();
            } else {
                panic!("Tried to access a value in a scope that is not an ancestor of the current provider node");
            }
        }

        return current;
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ProviderNode {
    pub scope: Scope,
    pub parent: Option<Rc<ProviderNode>>,
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

type ValueProvider<'a, T> = RefCell<HashMap<Rc<ProviderNode>, ProvidedValue<'a, T>>>;

impl<'a, T: Display> DataProvider<'a, T> {
    pub fn new(init_value: &'a dyn Fn() -> T) -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            init_value,
        }
    }
    pub fn get_value(&self, provider: Rc<ProviderNode>) -> Rc<T> {
        let initialized = { self.values.borrow().contains_key(&*provider.clone()) };

        if initialized {
            return Rc::clone(&self.values.borrow().get(&provider).unwrap().current);
        }

        {
            let new_value = ProvidedValue::new((self.init_value)());
            println!("Initializing value: {}", new_value.current);
            let mut values = self.values.borrow_mut();
            values.insert(provider.clone(), new_value);
        }
        return Rc::clone(&self.values.borrow().get(&provider.clone()).unwrap().current);
    }
    pub fn set_value(&self, provider: Rc<ProviderNode>, value: T) {
        let initialized = { self.values.borrow().contains_key(&provider) };

        if initialized {
            self.values.borrow_mut().get_mut(&provider).unwrap().current = Rc::new(value);
        } else {
            let new_value = ProvidedValue::new(value);
            self.values.borrow_mut().insert(provider, new_value);
        }
    }

    pub fn delete(&self, provider: Rc<ProviderNode>) {
        self.notify_dependents(provider.clone());
        self.values.borrow_mut().remove_entry(&provider);
    }

    pub fn attach_dependent(&self, provider: Rc<ProviderNode>, dependent: &'a dyn Dependent) {
        self.values
            .borrow()
            .get(&*provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .push(dependent);
    }

    pub fn notify_dependents(&self, provider: Rc<ProviderNode>) {
        self.values
            .borrow()
            .get(&provider)
            .unwrap()
            .dependents
            .borrow()
            .iter()
            .for_each(|d| d.destroy());
        self.values
            .borrow()
            .get(&provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .drain(0..);
    }
}
