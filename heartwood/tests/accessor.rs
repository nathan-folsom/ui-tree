mod common;

use std::rc::Rc;

use crate::common::TestDependent;
use heartwood::{
    accessor::Accessor,
    common::{Read, Write},
    provider::{ProviderNode, ProviderTree, Scope, GLOBAL_SCOPE},
    root::*,
};

#[test]
fn should_access_current_value() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let on_change = |_v: i32| {};
    let accessor = Accessor::new(&root, &on_change);

    assert_eq!(*accessor.current, 25);
}

#[test]
fn should_call_callback_on_change() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let mut update_count = 0;
    {
        let on_change = |v: i32| update_count += 1;
        let accessor = Accessor::new(&root, &on_change);

        root.set(30);

        assert_eq!(*accessor.current, 30);
    }

    assert_eq!(update_count, 1);
}
