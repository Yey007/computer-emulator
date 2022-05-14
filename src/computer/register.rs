use std::ops::{Add, Sub};
use ux::u2;

pub struct Register<T> {
    value: T,
}

impl<T: Copy + From<u2> + Add<Output = T> + Sub<Output = T>> Register<T> {
    pub fn new() -> Self {
        Register {
            value: T::try_from(u2::new(0)).unwrap(),
        }
    }

    pub fn store(&mut self, value: T) {
        self.value = value
    }

    pub fn load(&self) -> T {
        self.value
    }

    pub fn increment(&mut self) {
        self.value = self.value + T::try_from(u2::new(1)).unwrap()
    }

    pub fn decrement(&mut self) {
        self.value = self.value - T::try_from(u2::new(1)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let r: Register<u8> = Register::new();
        assert_eq!(r.value, 0)
    }

    #[test]
    fn store() {
        let mut r: Register<u8> = Register::new();
        r.store(1);
        assert_eq!(r.value, 1)
    }

    #[test]
    fn load() {
        let mut r: Register<u8> = Register::new();
        r.value = 10;
        assert_eq!(r.load(), 10)
    }

    #[test]
    fn increment() {
        let mut r: Register<u8> = Register::new();
        r.value = 10;
        r.increment();
        assert_eq!(r.value, 11)
    }

    #[test]
    fn decrement() {
        let mut r: Register<u8> = Register::new();
        r.value = 10;
        r.decrement();
        assert_eq!(r.value, 9)
    }
}
