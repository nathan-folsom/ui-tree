use std::cell::RefCell;
use std::collections::HashMap;
use crate::v2::provider_tree::{Dependent, ProviderNode};

pub struct DataProvider<T: 'static> {
    values: RefCell<Option<HashMap<&'static ProviderNode, ProvidedValue<T>>>>,
    init_value: &'static dyn Fn() -> T,
}

impl<T> DataProvider<T> {
   pub const fn new(init_value: &'static dyn Fn() -> T) -> Self {
       Self {
           values: RefCell::new(None),
           init_value,
       }
   }
    fn init(&self) {
        *self.values.borrow_mut() = Some(HashMap::new());
    }
}

pub struct ProvidedValue<T> {
    pub current: T,
    pub dependents: RefCell<Vec<&'static dyn Dependent>>,
}

#[test]
fn should_construct_static() {
    fn init() -> i32 { 25 }
    const provider: DataProvider<i32> = DataProvider::new(&init);
}