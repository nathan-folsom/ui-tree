mod common;

use crate::common::TestDependent;
use heartwood::{common::*, derived::*, provider::*, root::*};

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
    let derived_node = DerivedNode::new(&read, &write, &provider_tree, "derived");

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
    let derived_node = DerivedNode::new(&read, &write, &provider_tree, "derived");

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
    let derived_node = DerivedNode::new(&read, &write, &provider_tree, "derived");

    assert_eq!(*derived_node.get(), 50);

    root_node.set(60);

    assert_eq!(*derived_node.get(), 120);
}

#[test]
fn should_get_derived_chain() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let read1 = || *root.get() * 2;
    let write1 = |v: i32| root.set(v / 2);
    let derived1 = DerivedNode::new(&read1, &write1, &tree, "derived1");

    let read2 = || *derived1.get() / 2;
    let write2 = |v: i32| derived1.set(v * 2);
    let derived2 = DerivedNode::new(&read2, &write2, &tree, "derived2");

    assert_eq!(*derived2.get(), 25);
}

#[test]
fn should_propagate_from_root_to_chained_derived() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let read1 = || *root.get() * 2;
    let write1 = |v: i32| root.set(v / 2);
    let derived1 = DerivedNode::new(&read1, &write1, &tree, "derived1");

    let read2 = || *derived1.get() / 2;
    let write2 = |v: i32| derived1.set(v * 2);
    let derived2 = DerivedNode::new(&read2, &write2, &tree, "derived2");

    root.set(30);

    assert_eq!(*derived2.get(), 30);
}

#[test]
fn should_propagate_changes_to_chained_derived() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let read1 = || *root.get() * 2;
    let write1 = |v: i32| root.set(v / 2);
    let derived1 = DerivedNode::new(&read1, &write1, &tree, "derived1");

    let read2 = || *derived1.get() / 2;
    let write2 = |v: i32| derived1.set(v * 2);
    let derived2 = DerivedNode::new(&read2, &write2, &tree, "derived2");
    derived2.set(40);

    assert_eq!(*derived2.get(), 40);
}

#[test]
fn should_set_initialized_root_from_derived() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);

    let read1 = || *root.get() * 2;
    let write1 = |v: i32| root.set(v / 2);
    let derived1 = DerivedNode::new(&read1, &write1, &tree, "derived1");

    let read2 = || *derived1.get() / 2;
    let write2 = |v: i32| derived1.set(v * 2);
    let derived2 = DerivedNode::new(&read2, &write2, &tree, "derived2");

    derived2.get();

    root.set(30);

    derived2.set(40);

    assert_eq!(*derived2.get(), 40);
}
