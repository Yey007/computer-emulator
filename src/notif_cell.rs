use std::cell::RefCell;

pub struct NotifCell<T> {
    inner: RefCell<T>,
    listeners: Vec<Box<dyn Fn(T)>>
}

impl<T> NotifCell<T> {
    pub fn new(inner: T) -> Self {
        NotifCell {
            inner: RefCell::new(inner),
            listeners: Vec::new()
        }
    }

    pub fn get(&self) -> T {
        self.inner.borrow().clone()
    }

    pub fn set(&self, value: T) {
        self.inner.replace(value);
    }
    
    pub fn on_change(&mut self, listener: Box<dyn Fn(T)>) {
        self.listeners.push(listener);
    }
}