mod common;

use std::{cell::RefCell, rc::Rc};

use crate::common::TestDependent;
use heartwood::{accessor::Accessor, common::Write, provider::ProviderTree, root::*};

#[test]
fn should_access_current_value() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree, "Test Root Node");

    let on_change = |_v: Rc<i32>| {};
    let accessor = Accessor::new(&root, &on_change);

    assert_eq!(*accessor.current(), 25);
}

#[test]
fn should_call_callback_on_change() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree, "Test Root Node");

    struct Count {
        count: RefCell<i32>,
    }

    impl Count {
        pub fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }

        pub fn get(&self) -> i32 {
            *self.count.borrow()
        }
    }

    let counter = Count {
        count: RefCell::new(0),
    };
    {
        let on_change = |v: Rc<i32>| counter.increment();
        let accessor = Accessor::new(&root, &on_change);

        accessor.current(); // We have to access before setting to create dependency accessor ->
                            // root

        root.set(30);

        assert_eq!(*accessor.current(), 30);
    }

    assert_eq!(counter.get(), 1);
}
