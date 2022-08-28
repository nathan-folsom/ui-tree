pub mod tree {
    use std::any::Any;
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct NodeKey;

    pub struct RootNode<T> {
        pub initial_value: T,
    }

    impl<'a, T> Keyed for RootNode<T> {}

    trait Keyed {
        const KEY: NodeKey = NodeKey {};
    }

    pub struct Provider<'a> {
        nodes: HashMap<NodeKey, Value<'a, Box<dyn Any>>>,
        parent: Box<Option<Provider<'a>>>,
        scope: Option<Scope>,
    }

    pub struct Scope;

    impl<'a> Provider<'a> {
        pub fn new(parent: Option<Provider<'a>>, scope: Option<Scope>) -> Provider<'a> {
            Provider {
                nodes: HashMap::new(),
                parent: Box::new(parent),
                scope,
            }
        }
    }

    pub struct RootValue<'a, T> {
        current: T,
        subscribers: Vec<&'a DerivedNode>,
        provider: Provider<'a>,
        node: RootNode<T>,
    }

    impl<'a, T: Clone> RootValue<'a, T> {
        pub fn new(node: RootNode<T>, provider: Provider<'a>) -> RootValue<'a, T> {
            RootValue {
                current: node.initial_value.clone(),
                subscribers: Vec::new(),
                provider,
                node,
            }
        }
    }

    impl<T> Read<T> for RootValue<'_, T> {
        fn get(&self) -> &T {
            &self.current
        }
    }

    struct Value<'a, T> {
        current: T,
        subscriptions: HashMap<NodeKey, bool>,
        subscribers: Vec<&'a DerivedNode>,
        provider: Provider<'a>,
    }

    pub trait Read<T> {
        fn get(&self) -> &T;
    }

    pub trait Write<T> {
        fn set(&mut self, value: T);
    }

    pub struct DerivedNode {}

    pub fn create_global_provider() -> Provider<'static> {
        Provider::new(None, None)
    }
}
#[cfg(test)]
mod test {
    use crate::tree::{create_global_provider, Read, RootNode, RootValue};

    #[test]
    fn should_create_root() {
        let global_provider = create_global_provider();
        let root_node = RootNode { initial_value: 15 };
        let root_val = RootValue::new(root_node, global_provider);

        assert_eq!(*root_val.get(), 15);
    }
}
