use std::fmt::Display;

use heartwood::common::Dependent;

pub struct TestDependent {}

impl Dependent for TestDependent {
    fn destroy(&self) {}
}

impl Display for TestDependent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "test dependent")
    }
}
