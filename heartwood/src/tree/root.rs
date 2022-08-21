pub trait Read<T> {
    fn read(&self) -> T;
}

pub trait Write<T> {
    fn write(&self);
}

pub struct Root<T> {
    initial_value: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::tree::root::Read;

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
