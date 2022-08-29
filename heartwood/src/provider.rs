pub struct ProviderTree<'a> {
    root: ProviderNode<'a>,
}

impl ProviderTree<'static> {
    pub fn new() -> ProviderTree<'static> {
        ProviderTree {
            root: ProviderNode {
                scope: None,
                parent: None,
                children: Vec::new(),
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct ProviderNode<'a> {
    scope: Option<Scope>,
    parent: Option<Box<&'a ProviderNode<'a>>>,
    children: Vec<Box<ProviderNode<'a>>>,
}

impl ProviderTree<'_> {
    pub fn get_current(&self) -> &ProviderNode {
        &self.root
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct Scope();

pub struct ProvidedValue<T> {
    pub current: T,
    pub dependents: Vec<Box<dyn Dependent>>,
}

impl<T> ProvidedValue<T> {
    pub fn new(initial_value: T) -> ProvidedValue<T> {
        ProvidedValue {
            current: initial_value,
            dependents: Vec::new(),
        }
    }
}

pub trait Dependent {}
