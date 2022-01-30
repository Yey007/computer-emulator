pub struct Port<T> {
    value: T,
}

impl<T: Copy + Default> Port<T> {
    pub fn new() -> Self {
        Port { value: T::default() }
    }

    pub fn read(&self) -> T {
        self.value
    }

    pub fn write(&mut self, value: T) {
        self.value = value
    }
}
