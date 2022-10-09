use std::cell::RefCell;

pub struct Scope();

pub const GLOBAL_SCOPE: Scope = Scope();

pub struct ProviderTree {
    root: ProviderNode,
    dependent_stack: ProviderStack<&'static dyn Dependent>,
    scope_stack: ProviderStack<&'static Scope>,
    node_stack: ProviderStack<&'static ProviderNode>,
}

impl ProviderTree {
    pub const fn new() -> Self {
        Self {
            root: ProviderNode::new(GLOBAL_SCOPE),
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

pub struct ProviderNode {
    scope: Scope,
}

impl ProviderNode {
    pub const fn new(scope: Scope) -> Self {
        Self {
            scope,
        }
    }
}

pub trait Dependent {
    fn nudge(&self) {

    }
}

#[test]
fn should_create_const_tree() {
    const tree: ProviderTree = ProviderTree::new();
}