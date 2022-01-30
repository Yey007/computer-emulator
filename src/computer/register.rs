extern crate ux;

pub struct Register<T> {
    pub value: T
}

impl<T: Default + Copy> Register<T> {
    pub fn new() -> Self {
        Register { value: T::default() }
    }
}
