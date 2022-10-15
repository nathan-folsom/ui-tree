use std::cell::RefCell;

pub struct Scope();

pub const GLOBAL_SCOPE: Scope = Scope();

pub struct ProviderTree {
    root: ProviderNode<'static>,
    dependent_stack: ProviderStack<&'static dyn Dependent>,
    scope_stack: ProviderStack<&'static Scope>,
    node_stack: ProviderStack<&'static ProviderNode<'static>>,
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

struct ProviderStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> ProviderStack<T> {
    pub const fn new() -> Self {
        Self {
            stack: RefCell::new(Vec::new())
        }
    }
}

pub struct ProviderNode<'a> {
    scope: &'static Scope,
    parent: Option<&'a ProviderNode<'a>>,
    children: Vec<ProviderNode<'a>>,
}

impl<'a> ProviderNode<'a> {
    pub const fn new(scope: &Scope, parent: Option<&'a ProviderNode<'a>>) -> Self {
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
