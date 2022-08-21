pub struct Root<T> {
    pub initial_value: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::tree::node::Read;
        struct TestRead {
            value: u8,
        }

        impl Read<u8> for TestRead {
            fn read(&self) -> u8 {
                self.value
            }
        }

        let readable = TestRead { value: 5 };

        assert_eq!(readable.value, readable.read());
    }
}
