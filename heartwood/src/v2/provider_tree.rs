use std::cell::RefCell;

pub struct Scope();

pub const GLOBAL_SCOPE: Scope = Scope();

pub struct ProviderTree {
    pub root: ProviderNode,
    pub dependent_stack: ProviderStack<&'static dyn Dependent>,
    pub scope_stack: ProviderStack<&'static Scope>,
    pub node_stack: ProviderStack<&'static ProviderNode>,
}

impl ProviderTree {
    pub const fn new() -> Self {
        Self {
            root: ProviderNode::new(&GLOBAL_SCOPE, None),
            dependent_stack: ProviderStack::new(),
            scope_stack: ProviderStack::new(),
            node_stack: ProviderStack::new(),
        }
    }
}

pub struct ProviderStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> ProviderStack<T> {
    pub const fn new() -> Self {
        Self {
            stack: RefCell::new(Vec::new())
        }
    }
}

pub struct ProviderNode {
    scope: &'static Scope,
    parent: Option<&'static ProviderNode>,
    children: Vec<ProviderNode>,
}

impl ProviderNode {
    pub const fn new(scope: &'static Scope, parent: Option<&'static ProviderNode>) -> Self {
        Self {
            scope,
            parent,
            children: Vec::new(),
        }
    }
}

pub trait Dependent {
    fn nudge(&self) {

    }
}
