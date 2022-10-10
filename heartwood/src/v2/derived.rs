use crate::v2::data_provider::DataProvider;
use crate::v2::provider_tree::ProviderTree;

pub struct DerivedNode<T: 'static, U: 'static> {
    write: &'static dyn Fn(U),
    provider_tree: &'static ProviderTree,
    provider: DataProvider<T>,
    debug_name: &'static str,
}

impl<T: 'static, U: 'static> DerivedNode<T, U> {
    pub fn new(
        read: &'static dyn Fn() -> T,
        write: &'static dyn Fn(U),
        provider_tree: &'static ProviderTree,
        debug_name: &'static str,
    ) -> Self {
        Self {
            write,
            provider_tree,
            provider: DataProvider::new(read),
            debug_name,
        }
    }
}
