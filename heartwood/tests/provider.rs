mod common;

use crate::common::TestDependent;
use heartwood::{provider::ProviderTree, root::*};

#[test]
fn should_read_values_from_parent_provider_node() {
    let tree = ProviderTree::new(Some(&TestDependent {}));
    let root = RootNode::new(&|| 25, &tree);
}
