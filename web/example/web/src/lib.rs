mod utils;

use heartwood::{
    provider::{ProviderTree, Scope},
    root::RootNode,
};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
pub fn init_tree() {
    let provider_tree = ProviderTree::new(None);
    let counter_root = RootNode::new(&|| 0, &provider_tree);
    const SCOPEY_DOPE: Scope = Scope::new("Web Example Scope");
    let dbg = JsValue::from_str(&format!("{SCOPEY_DOPE:?}"));
    log_1(&dbg);
}
