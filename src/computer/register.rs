extern crate ux;

use crate::ux_extensions::extensions::{Decrement, Increment};

pub struct Register<T> {
    value: T,
}

impl<T: Default + Copy + Increment + Decrement> Register<T> {
    pub fn new() -> Self {
        Register {
            value: T::default(),
        }
    }

    pub fn store(&mut self, value: T) {
        self.value = value
    }

    pub fn load(&self) -> T {
        self.value
    }

    pub fn increment(&mut self) {
        self.value.increment()
    }

    pub fn decrement(&mut self) {
        self.value.decrement()
    }
}
