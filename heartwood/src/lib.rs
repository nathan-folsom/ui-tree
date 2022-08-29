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
        let provider_tree = ProviderTree::new();
        let mut root_node = RootNode::new(15, &provider_tree);

        assert_eq!(*root_node.get(), 15);
    }

    #[test]
    fn should_set_root() {
        let provider_tree = ProviderTree::new();
        let mut root_node = RootNode::new(15, &provider_tree);

        root_node.set(25);

        assert_eq!(*root_node.get(), 25);
    }

    #[test]
    fn should_create_derived() {
        let provider_tree = ProviderTree::new();
        let root_node = RootNode::new(25, &provider_tree);
        //        let derived_node = DerivedNode::new(|| root_node.get(), |v: i32| {});
    }

    #[test]
    fn should_get_derived() {
        let provider_tree = ProviderTree::new();
    }
}
