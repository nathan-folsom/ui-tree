mod common;

use std::rc::Rc;

use crate::common::TestDependent;
use heartwood::{
    common::{Read, Write},
    derived::DerivedNode,
    provider::{ProviderNode, ProviderTree, Scope, GLOBAL_SCOPE},
    root::*,
};

#[test]
fn should_get_and_set_root_in_scoped_provider() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    const LOCAL_SCOPE: Scope = Scope::new("Root Scoped Local Scope");
    let node = ProviderNode {
        scope: LOCAL_SCOPE,
        parent: Some(tree.root.clone()),
    };
    tree.set_current(Rc::new(node));

    root.setp(30, &GLOBAL_SCOPE);
    let local_value = root.get();
    let global_value = root.getp(&GLOBAL_SCOPE);

    println!("Local: {}, Global: {}", local_value, global_value);

    assert_eq!(*local_value, 25);
    assert_eq!(*global_value, 30);
}

#[test]
fn should_get_and_set_derived_in_scoped_provider() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);
    const LOCAL_SCOPE: Scope = Scope::new("Derived Scoped Local Scope");
    let node = ProviderNode {
        scope: LOCAL_SCOPE,
        parent: Some(tree.root.clone()),
    };
    tree.set_current(Rc::new(node));

    let read = || *root.get();
    let write = |v: i32| root.set(v);
    let derived = DerivedNode::new(&read, &write, &tree, "derived");

    derived.setp(30, &GLOBAL_SCOPE);

    let local_provided = derived.get();
    let global_provided = derived.getp(&GLOBAL_SCOPE);

    println!("Local: {}, Global: {}", local_provided, global_provided);

    assert_eq!(*local_provided, 25);
    assert_eq!(*global_provided, 30);
}
