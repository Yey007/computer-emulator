extern crate ux;

pub struct Register<T: Into<u64> + Default> { // Allows any unsigned integer type
    pub value: T
}

impl<T: Into<u64> + Default> Register<T> {
    pub fn new() -> Self {
        Register { value: T::default() }
    }
}
