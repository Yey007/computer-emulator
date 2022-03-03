extern crate ux;

pub struct Register<T> {
    value: T,
}

impl<T: Default + Copy> Register<T> {
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
}
