use crate::v2::data_provider::DataProvider;
use crate::v2::provider_tree::ProviderTree;

pub struct RootNode<T: 'static> {
    provider: DataProvider<T>,
    provider_tree: &'static ProviderTree,
    debug_name: &'static str,
}

impl<T: 'static> RootNode<T> {
    pub const fn new(init: &'static dyn Fn() -> T, provider_tree: &'static ProviderTree, debug_name: &'static str) -> Self {
        Self {
            provider_tree,
            provider: DataProvider::new(init),
            debug_name,
        }
    }
}
