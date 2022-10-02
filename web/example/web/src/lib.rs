mod utils;

use heartwood::{
    accessor::Accessor,
    provider::{ProviderTree, Scope},
    root::RootNode,
};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

const APP_PROVIDER: ProviderTree<'static> = ProviderTree::new(None);

#[wasm_bindgen]
pub fn init_tree() {
    let appState = AppState::new(&APP_PROVIDER);

    let counter = Counter {
        root: RootNode::new(&|| 0, &APP_PROVIDER, "Counter Root"),
    };

    const SCOPEY_DOPE: Scope = Scope::new("Web Example Scope");
    let dbg = JsValue::from_str(&format!("{SCOPEY_DOPE:?}"));
    log_1(&dbg);
}

#[wasm_bindgen]
struct AppState {
    counter: Counter<'static>,
}

impl AppState {
    pub fn new(provider_tree: &'static ProviderTree<'static>) -> Self {
        Self {
            counter: Counter::<'static>::new(&provider_tree),
        }
    }
}

struct Counter<'a> {
    root: RootNode<'a, i32>,
}

impl<'a> Counter<'a> {
    pub fn new(provider_tree: &'a ProviderTree<'a>) -> Self {
        Self {
            root: RootNode::new(&|| 0, provider_tree, "Counter Root"),
        }
    }

    pub fn get_accessor(&'a self) {
        Accessor::new(&self.root, &|v| {});
    }
}
