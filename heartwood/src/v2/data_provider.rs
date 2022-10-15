use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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

    const EXPECT_INIT: &'static str = "Values map should be initialized";

    pub fn get_value(&self, provider: &ProviderNode) -> Rc<T> {
        let initialized = { self.values.borrow().expect(DataProvider::EXPECT_INIT).contains_key(provider) };

        if initialized {
            return Rc::clone(&self.values.borrow().expect(DataProvider::EXPECT_INIT).get(provider).unwrap().current);
        }

        {
            let new_value = ProvidedValue::new((self.init_value)());
            println!("Initializing value: {}", new_value.current);
            let mut values = self.values.borrow_mut().expect(DataProvider::EXPECT_INIT);
            values.expect(DataProvider::EXPECT_INIT).insert(provider, new_value);
        }
        return Rc::clone(&self.values.borrow().expect(DataProvider::EXPECT_INIT).get(&provider.clone()).unwrap().current);
    }

    pub fn set_value(&self, provider: &ProviderNode, value: T) {
        let initialized = { self.values.borrow().expect(DataProvider::EXPECT_INIT).contains_key(&provider) };

        if initialized {
            self.values.borrow_mut().expect(DataProvider::EXPECT_INIT).get_mut(&provider).unwrap().current = Rc::new(value);
        } else {
            let new_value = ProvidedValue::new(value);
            self.values.borrow_mut().expect(DataProvider::EXPECT_INIT).insert(provider, new_value);
        }
    }

    pub fn attach_dependent(&self, provider: &ProviderNode, dependent: &dyn Dependent) {
        self.values
            .borrow()
            .expect(DataProvider::EXPECT_INIT)
            .get(&provider)
            .unwrap()
            .dependents
            .borrow_mut()
            .push(dependent);
    }

    pub fn delete(&self, provider: &ProviderNode) {
        let dependents = self.clone_dependents(provider);

        self.values.borrow_mut().expect(DataProvider::EXPECT_INIT).remove_entry(&provider);

        Self::nudge_dependents(dependents);
    }

    pub fn notify_dependents(&self, provider: &ProviderNode) {
        let values = &self.values;
        let values_ref = values.borrow().expect(DataProvider::EXPECT_INIT);
        let current_node = values_ref.get(&provider).unwrap();

        let local_deps = self.clone_dependents(provider);

        Self::nudge_dependents(local_deps);

        *current_node.dependents.borrow_mut() = vec![];
    }

    fn nudge_dependents(dependents: Vec<&dyn Dependent>) {
        let mut dependents_iter = dependents.into_iter();

        while let Some(d) = dependents_iter.next() {
            d.nudge();
        }
    }

    fn clone_dependents(&self, provider: &ProviderNode) -> Vec<&dyn Dependent> {
        let values = &self.values;
        let values_ref = values.borrow().expect(DataProvider::EXPECT_INIT);
        let value = values_ref.get(&provider).unwrap();

        let mut local_deps = vec![];

        {
            let current_dependents = value.dependents.borrow_mut();
            let mut current_dependents_iter = current_dependents.iter();

            while let Some(v) = current_dependents_iter.next() {
                local_deps.push(*v);
            }
        };

        local_deps
    }
}

pub struct ProvidedValue<T> {
    pub current: T,
    pub dependents: RefCell<Vec<&'static dyn Dependent>>,
}
