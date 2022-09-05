mod common;
mod derived;
mod provider;
mod root;

#[cfg(test)]
mod test {
    use crate::common::*;
    use crate::derived::*;
    use crate::provider::*;
    use crate::root::*;

    #[test]
    fn should_create_root<'a>() {
        let provider_tree = ProviderTree::new(Some(&TestDependent {}));
        let root_node = RootNode::new(&|| 15, &provider_tree);

        assert_eq!(*root_node.get(), 15);
    }

    #[test]
    fn should_set_root() {
        let provider_tree = ProviderTree::new(Some(&TestDependent {}));
        let root_node = RootNode::new(&|| 15, &provider_tree);

        root_node.set(25);

        assert_eq!(*root_node.get(), 25);
    }

    #[test]
    fn should_create_derived() {
        let provider_tree = ProviderTree::new(Some(&TestDependent {}));
        let root_node = RootNode::new(&|| 25, &provider_tree);
        let read = || *root_node.get() * 2;
        let write = |v: i32| {
            root_node.set(v);
        };
        let derived_node = DerivedNode::new(&read, &write, &provider_tree);

        assert_eq!(*derived_node.get(), 50);
    }

    #[test]
    fn should_get_derived() {
        let provider_tree = ProviderTree::new(Some(&TestDependent {}));
        let root_node = RootNode::new(&|| 25, &provider_tree);
        let read = || *root_node.get() * 2;
        let write = |v: i32| {
            root_node.set(v / 2);
        };
        let derived_node = DerivedNode::new(&read, &write, &provider_tree);

        derived_node.set(60);

        assert_eq!(*root_node.get(), 30);
        assert_eq!(*derived_node.get(), 60);
    }

    #[test]
    fn should_update_derived_when_root_updates() {
        let provider_tree = ProviderTree::new(Some(&TestDependent {}));
        let root_node = RootNode::new(&|| 25, &provider_tree);

        let read = || *root_node.get() * 2;
        let write = |_v: i32| {};
        let derived_node = DerivedNode::new(&read, &write, &provider_tree);

        assert_eq!(*derived_node.get(), 50);

        root_node.set(60);

        assert_eq!(*derived_node.get(), 120);
    }

    struct TestDependent {}

    impl Dependent for TestDependent {
        fn destroy(&self) {
            println!("Destroying outdated dependent value");
        }
    }
}
