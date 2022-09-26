use crate::common::Dependent;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

pub const GLOBAL_SCOPE: Scope = Scope::new("Global Scope");

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
            scope: &GLOBAL_SCOPE,
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
        println!(
            "Getting current node, scope is '{:?}', working scope is '{:?}'",
            current.scope, scope
        );

        loop {
            if current.scope == scope {
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

    pub fn set_current(&self, provider_node: Rc<ProviderNode>) {
        self.scope_stack.borrow_mut().push(provider_node.scope);
        *self.current.borrow_mut() = Some(provider_node.clone());
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ProviderNode {
    pub scope: &'static Scope,
    pub parent: Option<Rc<ProviderNode>>,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Scope {
    debug_name: &'static str,
}

impl Scope {
    pub const fn new(debug_name: &'static str) -> Self {
        Self { debug_name }
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: {}", self.debug_name)
    }
}

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
        let initialized = { self.values.borrow().contains_key(&provider.clone()) };

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

    pub fn attach_dependent(&self, provider: Rc<ProviderNode>, dependent: &'a dyn Dependent) {
        self.values
            .borrow()
            .get(&provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .push(dependent);
    }

    pub fn delete(&self, provider: Rc<ProviderNode>) {
        let dependents = self.clone_dependents(provider.clone());

        self.values.borrow_mut().remove_entry(&provider);

        Self::nudge_dependents(dependents);
    }

    pub fn notify_dependents(&self, provider: Rc<ProviderNode>) {
        let values = &self.values;
        let values_ref = values.borrow();
        let current_node = values_ref.get(&provider).unwrap();

        let local_deps = self.clone_dependents(provider);

        Self::nudge_dependents(local_deps);

        *current_node.dependents.borrow_mut() = vec![];
    }

    fn nudge_dependents(dependents: Vec<&dyn Dependent>) {
        let mut dependents_iter = dependents.into_iter();

        while let Some(d) = dependents_iter.next() {
            d.nudge();
        }
    }

    fn clone_dependents(&self, provider: Rc<ProviderNode>) -> Vec<&dyn Dependent> {
        let values = &self.values;
        let values_ref = values.borrow();
        let value = values_ref.get(&provider).unwrap();

        let mut local_deps = vec![];

        {
            let current_dependents = value.dependents.borrow_mut();
            let mut current_dependents_iter = current_dependents.iter();

            while let Some(v) = current_dependents_iter.next() {
                local_deps.push(*v);
            }
        };

        local_deps
    }
}

pub trait Provided<'a> {
    fn get_tree(&self) -> &'a ProviderTree;
}
