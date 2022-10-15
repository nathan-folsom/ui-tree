use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::thread::scope;

#[derive(Hash, PartialEq, Eq)]
pub struct Scope(&'static str);

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Scope(debug_name) = self;
        write!(f, "{}", debug_name)
    }
}

pub const GLOBAL_SCOPE: Scope = Scope("Global Scope");

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

    fn get_base(&self) -> &ProviderNode {
        self.node_stack.stack.borrow().last().expect("Tried to get current node, but no node was found in stack")
    }

    pub fn get_current(&self) -> &ProviderNode {
        let mut current = self.get_base();
        let scope = &**self.scope_stack.stack.borrow().last().unwrap();
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
        self.scope_stack.stack.borrow_mut().push(provider_node.scope);
        *self.current.borrow_mut() = Some(provider_node.clone());
    }
}

pub struct ProviderStack<T> {
    pub stack: RefCell<Vec<T>>,
}

impl<T> ProviderStack<T> {
    pub const fn new() -> Self {
        Self {
            stack: RefCell::new(Vec::new())
        }
    }

    pub fn act(&self, value: T, callback: fn()) {
        {
            self.stack.borrow_mut().push(value);
        }

        callback();

        {
            self.stack.borrow_mut().pop();
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
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

pub trait Provided {
    fn get_tree(&self) -> &ProviderTree;
}
